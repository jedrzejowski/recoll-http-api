IMAGE_NAME=$(shell cargo pkgid | cut -d# -f2 | cut -d@ -f1)
IMAGE_TAG=$(shell git rev-parse --abbrev-ref HEAD)-$(shell date +"%Y%m%d")-$(shell git rev-parse --short HEAD)

docker-build:
	docker build . --tag ${IMAGE_PREFIX}${IMAGE_NAME}:${IMAGE_TAG}
docker-push:
	docker push ${IMAGE_PREFIX}${IMAGE_NAME}:${IMAGE_TAG}



docker-build-recoll-dev:
	docker build . --tag recoll --target recoll-builder