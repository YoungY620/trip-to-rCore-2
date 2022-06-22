DOCKER_NAME ?= dinghao188/rcore-tutorial
.PHONY: docker build_docker

docker:
	docker run --name rcore -d --rm -it --mount type=bind,source=$(shell pwd),destination=/mnt ${DOCKER_NAME}

build_docker: 
	docker build -t ${DOCKER_NAME} .
enter_docker:
	docker exec -it rcore bash 
fmt:
	cd easy-fs; cargo fmt; cd ../easy-fs-fuse cargo fmt; cd ../os ; cargo fmt; cd ../user; cargo fmt; cd ..

clear:
	@docker stop rcore
	@docker rm rcore
