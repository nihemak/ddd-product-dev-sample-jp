-- Add down migration script here

-- Drop the trigger first
DROP TRIGGER IF EXISTS set_timestamp_reservations ON reservations;

-- Drop the trigger function
DROP FUNCTION IF EXISTS trigger_set_timestamp();

-- Drop the reservation_products table
DROP TABLE IF EXISTS reservation_products;

-- Drop the reservations table
DROP TABLE IF EXISTS reservations;
