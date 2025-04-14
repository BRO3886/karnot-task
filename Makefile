dbg:
	@RUST_LOG=actix_web=debug cargo run

run:
	@RUST_LOG=actix_web=info cargo run

migrate:
	@diesel migration run

migrate-rollback:
	@diesel migration rollback

migrate-reset:
	@diesel migration reset