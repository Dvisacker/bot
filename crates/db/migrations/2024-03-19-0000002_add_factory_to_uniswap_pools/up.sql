ALTER TABLE uni_v2_pools
ADD COLUMN factory VARCHAR;

ALTER TABLE uni_v3_pools
ADD COLUMN factory VARCHAR;

ALTER TABLE uni_v3_pools
ADD COLUMN filtered BOOLEAN;