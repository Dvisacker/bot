// SPDX-License-Identifier: MIT
pragma solidity 0.8.26;

import "forge-std/Test.sol";
import {WETH} from "../lib/solmate/src/tokens/WETH.sol";
import "../src/BatchExecutor.sol";
import "forge-std/console.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../src/interfaces/IUniswapV3Router.sol";
import "../src/interfaces/IUniswapV2Router.sol";
import "../src/interfaces/IAerodromeRouter.sol";
import "../lib/v3-core/contracts/interfaces/IUniswapV3Pool.sol";

contract BatchExecutorTest is Test {
    address deployer = makeAddr("deployer");

    WETH weth = WETH(payable(0x4200000000000000000000000000000000000006));
    ERC20 dai = ERC20(0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb);
    ERC20 usdc = ERC20(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913);
    ERC20 usdt = ERC20(0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2);

    BatchExecutor public executor;
    address owner = address(this);
    IUniswapV3Router public uniswapV3Router = IUniswapV3Router(0x2626664c2603336E57B271c5C0b26F421741e481);
    IUniswapV2Router public uniswapV2Router = IUniswapV2Router(0x4752ba5DBc23f44D87826276BF6Fd6b1C372aD24);
    IAerodromeRouter aerodromeRouter = IAerodromeRouter(0xcF77a3Ba9A5CA399B7c97c74d54e5b1Beb874E43);

    // V3 factory address
    address factory = 0x33128a8fC17869897dcE68Ed026d694621f6FDfD;
    // V3 init code hash
    bytes32 initCodeHash = 0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54;

    function setUp() public {
        vm.createSelectFork((vm.envString("BASE_RPC_URL")));
        executor = new BatchExecutor(deployer);
        vm.deal(deployer, 10 ether);
    }

    struct CallbackContext {
        uint32 dataIndex;
        address sender;
    }

    function encodedContext(CallbackContext memory context) public pure returns (bytes32) {
        // Convert uint32 to bytes32, right-aligned
        bytes32 dataIndexBytes32 = bytes32(uint256(context.dataIndex));

        // Convert address to bytes32, right-aligned
        bytes32 senderBytes32 = bytes32(uint256(uint160(context.sender)));

        bytes32 shiftedDataIndexBytes32 = dataIndexBytes32 << 160;
        // Shift dataIndex left by 160 bits (20 bytes) and combine with sender
        bytes32 result = (shiftedDataIndexBytes32 & 0xffffffffffffffffffffffff0000000000000000000000000000000000000000)
            | (senderBytes32 & 0x000000000000000000000000ffffffffffffffffffffffffffffffffffffffff);

        return result;
    }

    function getCreate2Address(address from, bytes32 salt, bytes32 initCodeHash) public pure returns (address addr) {
        /// @notice Compute CREATE2 address
        /// @dev Follows the same logic as the typescript version but in Solidity
        /// @param from The address deploying the contract
        /// @param salt The salt used in the CREATE2 deployment
        /// @param initCodeHash The hash of the contract's init code

        // Pack and hash according to CREATE2 specification
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0xff), // Fixed prefix
                from, // Creator address
                salt, // Randomizing salt
                initCodeHash // Init code hash
            )
        );

        // Convert last 20 bytes of hash to address
        addr = address(uint160(uint256(hash)));
    }

    function computeV3PoolAddress(bytes32 initCodeHash, address factory, address tokenA, address tokenB, uint24 fee)
        public
        pure
        returns (address)
    {
        // Sort tokens
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);

        // Compute the salt from packed tokens and fee
        bytes32 salt = keccak256(abi.encode(token0, token1, fee));

        return getCreate2Address(factory, salt, initCodeHash);
    }

    function buildCall(
        address target,
        uint256 value,
        bytes memory callData,
        CallbackContext memory context,
        BatchExecutor.DynamicCall[] memory dynamicCalls
    ) public returns (bytes memory) {
        bytes32 contextBytes = encodedContext(context);
        return abi.encodeWithSelector(executor.singlecall.selector, target, value, contextBytes, callData, dynamicCalls);
    }

    function testWrapEth() public {
        uint256 amountIn = 1 ether;
        bytes memory depositWethCallData = abi.encodeWithSelector(weth.deposit.selector);
        bytes memory callData = buildCall(
            address(weth),
            amountIn,
            depositWethCallData,
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes[] memory callDataArray = new bytes[](1);
        callDataArray[0] = callData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }

    function testWrapUnwrapEth() public {
        uint256 amountIn = 1 ether;
        bytes memory depositWethCallData = buildCall(
            address(weth),
            amountIn,
            abi.encodeWithSelector(weth.deposit.selector),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes memory withdrawWethCallData = buildCall(
            address(weth),
            0,
            abi.encodeWithSelector(weth.withdraw.selector, amountIn),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes[] memory callDataArray = new bytes[](2);
        callDataArray[0] = depositWethCallData;
        callDataArray[1] = withdrawWethCallData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }

    function testUniswapV3SwapExact() public {
        uint256 amountIn = 1 ether;
        bytes memory depositWethCallData = buildCall(
            address(weth),
            amountIn,
            abi.encodeWithSelector(weth.deposit.selector),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes memory approveErc20CallData = buildCall(
            address(weth),
            0,
            abi.encodeWithSelector(weth.approve.selector, uniswapV3Router, amountIn),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        bytes memory swapCallData = buildCall(
            address(uniswapV3Router),
            0,
            abi.encodeWithSelector(
                uniswapV3Router.exactInputSingle.selector,
                IUniswapV3Router.ExactInputSingleParams({
                    tokenIn: address(weth),
                    tokenOut: address(usdc),
                    fee: 3000,
                    recipient: address(executor),
                    amountIn: amountIn,
                    amountOutMinimum: 0,
                    sqrtPriceLimitX96: 0
                })
            ),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        bytes[] memory callDataArray = new bytes[](3);
        callDataArray[0] = depositWethCallData;
        callDataArray[1] = approveErc20CallData;
        callDataArray[2] = swapCallData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }

    function testUniswapV3SwapAll() public {
        uint256 amountIn = 1 ether;

        bytes memory depositWethCallData = buildCall(
            address(weth),
            amountIn,
            abi.encodeWithSelector(weth.deposit.selector),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes memory approveErc20CallData = buildCall(
            address(weth),
            0,
            abi.encodeWithSelector(weth.approve.selector, uniswapV3Router, amountIn),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        BatchExecutor.DynamicCall memory balanceOf = BatchExecutor.DynamicCall({
            to: address(weth),
            data: abi.encodeWithSelector(weth.balanceOf.selector, address(executor)),
            offset: 4 + 32 * 4,
            length: 32,
            resOffset: 0
        });

        BatchExecutor.DynamicCall[] memory dynamicCalls = new BatchExecutor.DynamicCall[](1);
        dynamicCalls[0] = balanceOf;

        bytes memory swapCallData = buildCall(
            address(uniswapV3Router),
            0,
            abi.encodeWithSelector(
                uniswapV3Router.exactInputSingle.selector,
                IUniswapV3Router.ExactInputSingleParams({
                    tokenIn: address(weth),
                    tokenOut: address(usdc),
                    fee: 3000,
                    recipient: address(executor),
                    amountIn: 0, // replaced by dynamic call
                    amountOutMinimum: 0,
                    sqrtPriceLimitX96: 0
                })
            ),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            dynamicCalls
        );

        bytes[] memory callDataArray = new bytes[](3);
        callDataArray[0] = depositWethCallData;
        callDataArray[1] = approveErc20CallData;
        callDataArray[2] = swapCallData;

        vm.startPrank(deployer);
        executor.batchCall{value: amountIn}(callDataArray);
        vm.stopPrank();
    }

    function testUniswapV3FlashLoan() public {
        address pool_address = computeV3PoolAddress(initCodeHash, factory, address(weth), address(usdc), 3000);

        uint256 amountUsdc = 3000 * 10 ** 6;
        uint256 amountWeth = 1 ether;
        IUniswapV3Pool pool = IUniswapV3Pool(pool_address);

        deal(address(usdc), address(executor), 100000 * 1e6);
        deal(address(weth), address(executor), 100 ether);

        uint256 amountWethWithPremium = amountWeth + (300 * amountWeth / 100000);
        uint256 amountUsdcWithPremium = amountUsdc + (300 * amountUsdc / 100000);

        bytes memory transfer0Calldata = buildCall(
            address(weth),
            0,
            abi.encodeWithSelector(weth.transfer.selector, address(pool), amountWethWithPremium),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );
        bytes memory transfer1Calldata = buildCall(
            address(usdc),
            0,
            abi.encodeWithSelector(usdc.transfer.selector, address(pool), amountUsdcWithPremium),
            CallbackContext({dataIndex: 0, sender: address(this)}),
            new BatchExecutor.DynamicCall[](0)
        );

        bytes[] memory callbackCalls = new bytes[](2);
        callbackCalls[0] = transfer0Calldata;
        callbackCalls[1] = transfer1Calldata;
        bytes memory callbackData = abi.encode(callbackCalls, "0x");

        CallbackContext memory context = CallbackContext({dataIndex: 2, sender: address(pool)});

        bytes memory flashCallData = buildCall(
            address(pool),
            0,
            abi.encodeWithSelector(pool.flash.selector, address(executor), amountWeth, amountUsdc, callbackData),
            context,
            new BatchExecutor.DynamicCall[](0)
        );

        bytes[] memory callDataArray = new bytes[](1);
        callDataArray[0] = flashCallData;

        vm.startPrank(deployer);
        executor.batchCall(callDataArray);
        vm.stopPrank();
    }
}
