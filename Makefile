include .env
export

dev-app:
	docker-compose --env-file .env up app-api $(filter-out $@,$(MAKECMDGOALS))

dev:
	docker-compose up $(filter-out $@,$(MAKECMDGOALS))

stop-app:
	@docker-compose down

create-migration:
	@docker exec tax-file-api bash ./scripts/migration.sh $(filter-out $@,$(MAKECMDGOALS))
run-migration:
	@docker exec -it tax-file-api bash -c "sqlx migrate run --source ./src/migrations"
	@docker exec -it tax-file-api bash -c "cargo sqlx prepare"
rollback-migration:
	@docker exec -it tax-file-api bash -c "sqlx migrate revert --source ./src/migrations"
	@docker exec -it tax-file-api bash -c "cargo sqlx prepare"
#tree:
	#docker exec -it tax-file-api bash -c "cargo modules generate tree"
prepare:
	@docker exec -it tax-file-api bash -c "cargo sqlx prepare"

test:
	@docker exec -it tax-file-api bash -c "cargo test"
