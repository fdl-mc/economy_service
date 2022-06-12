build:
	@docker build -t fdl-mc/api/economy .
run:
	@docker run fdl-mc/api/economy
deploy:
	@docker-compose up -d
