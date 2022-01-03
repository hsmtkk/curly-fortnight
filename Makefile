podman-build:
	podman build --tag hsmtkk/curly-fortnight .

podman-run:
	podman run --detach --env PORT=8000 --rm hsmtkk/curly-fortnight
