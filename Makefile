build:
		docker buildx build \
  --platform linux/amd64 \
  --target export \
  --output type=local,dest=./output \
  .
