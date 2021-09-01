start-nginx: cert.crt
	nginx -p . -c nginx.conf -e /dev/stderr

cert.crt:
	openssl req -nodes -new -x509 -newkey rsa:4096 -keyout cert.key -out cert.crt -days 365 -subj '/CN=localhost'
