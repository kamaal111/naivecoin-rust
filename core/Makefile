bootstrap:
	brew tap mongodb/brew
	brew update
	brew install mongodb-community@4.2

	cargo install cargo-watch

create-dev-database:
	sh scripts/create-dev-database.sh

start-dev-database:
	brew services start mongodb-community@4.2

stop-dev-database:
	brew services stop mongodb-community@4.2

enter-dev-database:
	mongo

start-dev-server:
	cargo watch -x run
