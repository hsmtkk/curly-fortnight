podman-build:
	podman build --tag hsmtkk/curly-fortnight .

podman-run:
	podman run --detach --env PORT=8000 --rm hsmtkk/curly-fortnight

heroku-container-login:
	heroku container:login

heroku-container-push:
	heroku container:push web

heroku-container-release:
	heroku container:release web
