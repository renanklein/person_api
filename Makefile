up:
	@docker-compose up -d --build
	@diesel migration run --database-url postgresql://localusr:localpass@localhost:5432/person

down:
	@docker-compose down

stop :
	@docker-compose stop