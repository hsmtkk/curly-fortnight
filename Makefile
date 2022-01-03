podman-build:
	podman build --file local.Dockerfile --tag hsmtkk/curly-fortnight .

podman-run:
	podman run --detach --env PORT=8000 --network=host --rm hsmtkk/curly-fortnight
