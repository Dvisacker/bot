use crate::models::{NewPool, Pool};
use crate::schema::pools;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

pub fn insert_pool(conn: &mut SqliteConnection, new_pool: &NewPool) -> Result<Pool, Error> {
    let result = diesel::insert_into(pools::table)
        .values(new_pool)
        .execute(conn);

    if result.is_err() {
        return Err(result.err().unwrap());
    }

    pools::table
        .order(pools::id.desc())
        .first(conn)
        .map_err(|_| Error::NotFound)
}

pub fn batch_insert_pools(
    conn: &mut SqliteConnection,
    new_pools: &Vec<NewPool>,
) -> Result<Vec<Pool>, Error> {
    conn.transaction(|conn| {
        let inserted_pools: Vec<Pool> = new_pools
            .iter()
            .map(|new_pool| {
                diesel::insert_into(pools::table)
                    .values(new_pool)
                    .execute(conn)?;

                pools::table.order(pools::id.desc()).first(conn)
            })
            .collect::<Result<Vec<Pool>, Error>>()?;

        Ok(inserted_pools)
    })
}

pub fn batch_upsert_pools(
    conn: &mut SqliteConnection,
    new_pools: &Vec<NewPool>,
) -> Result<Vec<Pool>, Error> {
    conn.transaction(|conn| {
        let inserted_pools: Vec<Pool> = new_pools
            .iter()
            .map(|new_pool| {
                diesel::insert_into(pools::table)
                    .values(new_pool)
                    .on_conflict((pools::chain, pools::address))
                    .do_update()
                    .set((
                        pools::factory_address.eq(&new_pool.factory_address),
                        pools::exchange_name.eq(&new_pool.exchange_name),
                        pools::exchange_type.eq(&new_pool.exchange_type),
                        pools::token_a.eq(&new_pool.token_a),
                        pools::token_a_symbol.eq(&new_pool.token_a_symbol),
                        pools::token_a_decimals.eq(new_pool.token_a_decimals),
                        pools::token_b.eq(&new_pool.token_b),
                        pools::token_b_symbol.eq(&new_pool.token_b_symbol),
                        pools::token_b_decimals.eq(new_pool.token_b_decimals),
                        pools::reserve_0.eq(&new_pool.reserve_0),
                        pools::reserve_1.eq(&new_pool.reserve_1),
                        pools::fee.eq(new_pool.fee),
                    ))
                    .execute(conn)?;

                pools::table
                    .filter(pools::chain.eq(&new_pool.chain))
                    .filter(pools::address.eq(&new_pool.address))
                    .first(conn)
            })
            .collect::<Result<Vec<Pool>, Error>>()?;

        Ok(inserted_pools)
    })
}

pub fn get_all_pools(conn: &mut SqliteConnection) -> Result<Vec<Pool>, Error> {
    pools::table.load::<Pool>(conn)
}

pub fn get_pool_by_address(conn: &mut SqliteConnection, pool_address: &str) -> Result<Pool, Error> {
    pools::table
        .filter(pools::address.eq(pool_address))
        .first(conn)
}

pub fn update_pool(
    conn: &mut SqliteConnection,
    pool_address: &str,
    updated_pool: &NewPool,
) -> Result<Pool, Error> {
    diesel::update(pools::table.filter(pools::address.eq(pool_address)))
        .set((
            pools::chain.eq(updated_pool.chain.clone()),
            pools::factory_address.eq(updated_pool.factory_address.clone()),
            pools::exchange_name.eq(updated_pool.exchange_name.clone()),
            pools::exchange_type.eq(updated_pool.exchange_type.clone()), // Add this line
            pools::token_a.eq(updated_pool.token_a.clone()),
            pools::token_a_symbol.eq(updated_pool.token_a_symbol.clone()),
            pools::token_a_decimals.eq(updated_pool.token_a_decimals),
            pools::token_b.eq(updated_pool.token_b.clone()),
            pools::token_b_symbol.eq(updated_pool.token_b_symbol.clone()),
            pools::token_b_decimals.eq(updated_pool.token_b_decimals),
            pools::reserve_0.eq(updated_pool.reserve_0.clone()),
            pools::reserve_1.eq(updated_pool.reserve_1.clone()),
            pools::fee.eq(updated_pool.fee),
        ))
        .execute(conn)?;

    get_pool_by_address(conn, pool_address)
}

pub fn delete_pool(conn: &mut SqliteConnection, pool_address: &str) -> Result<usize, Error> {
    diesel::delete(pools::table.filter(pools::address.eq(pool_address))).execute(conn)
}

pub fn get_pools(
    conn: &mut SqliteConnection,
    chain_name: Option<&str>,
    exchange_name: Option<&str>,
    exchange_type: Option<&str>,
    limit: Option<i64>,
    filtered: Option<bool>,
) -> Result<Vec<Pool>, Error> {
    let mut query = pools::table.into_boxed();

    if let Some(chain_name) = chain_name {
        query = query.filter(pools::chain.eq(chain_name));
    }

    if let Some(exchange_name) = exchange_name {
        query = query.filter(pools::exchange_name.eq(exchange_name));
    }

    if let Some(exchange_type) = exchange_type {
        query = query.filter(pools::exchange_type.eq(exchange_type));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(filtered) = filtered {
        query = query.filter(pools::filtered.eq(filtered));
    }

    query.load::<Pool>(conn)
}

// Add a new function to get pools by chain
pub fn get_pools_by_chain(
    conn: &mut SqliteConnection,
    chain_name: &str,
) -> Result<Vec<Pool>, Error> {
    pools::table
        .filter(pools::chain.eq(chain_name))
        .load::<Pool>(conn)
}

// Add a new function to get pools by exchange
pub fn get_pools_by_exchange(
    conn: &mut SqliteConnection,
    exchange_name: &str,
) -> Result<Vec<Pool>, Error> {
    pools::table
        .filter(pools::exchange_name.eq(exchange_name))
        .load::<Pool>(conn)
}

// pub fn get_pools(
//     conn: &mut SqliteConnection,
//     chain_name: &str,
//     exchange_name: &str,
//     exchange_type: &str,
//     limit: i64,
// ) -> Result<Vec<Pool>, Error> {
//     pools::table
//         .filter(pools::chain.eq(chain_name))
//         .filter(pools::exchange_name.eq(exchange_name))
//         .filter(pools::exchange_type.eq(exchange_type))
//         .limit(limit)
//         .load::<Pool>(conn)
// }

pub fn batch_update_filtered(
    conn: &mut SqliteConnection,
    pool_addresses: &[String],
    filtered: bool,
) -> QueryResult<usize> {
    diesel::update(pools::table)
        .filter(pools::address.eq_any(pool_addresses))
        .set(pools::filtered.eq(filtered))
        .execute(conn)
}

pub fn get_filtered_pools(
    conn: &mut SqliteConnection,
    chain_name: &str,
) -> Result<Vec<Pool>, Error> {
    pools::table
        .filter(pools::chain.eq(chain_name))
        .filter(pools::filtered.eq(true))
        .load::<Pool>(conn)
}