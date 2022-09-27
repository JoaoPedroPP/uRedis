# µRedis - A Rust Microservice to recod and read data in Redis

## About the Project

µRedis is a microservice that implement a connection pool on Redis Database. 

### Built with

This project was built with a some library listed on [Cargo.toml](Cargo.toml) file, however the main components are listed below.

1. Rust
2. Actix-Web
3. Redis-rs
4. Deadpool

<!-- ## Getting Started


### Prerequisites
### Installation -->
## Deployment

### 1. Docker

The simplest way is to build the image and store it in a registry, and then on the deploy paltform add the environment variables.

```bash
$ docker build -t image_name .
$ docker push image_name
```

However, you can create the `.env` file according to [env.sample](./env.sample) and add your credentials there, then uncomment the `COPY .env .` comamnd on the [Dockerfile](./Dockerfile), line 21. Build the image, store it in a registry service and deploy the docker image in the service of your preference.

There is another option, you can insert the environment varibales directly on the Dockerfile as shown below.
```dockerfile
ENV RUST_LOG=info

ENV REDIS_URL=YOUR_REDIS_URL
```

_Obs: It is highly recommended to avoid add the local environment variables in a file or inside the Dockerfile if you will store the docker image in a public registry as Docker Hub, you crendential would be exposed. Many deploy platforms have a secction where you can insert enviroment variables and this is safest option to proceed wit you choose to do it this way._

### 2. Kubernetes

This is repository is ready to deploy the application in kubernetes. You need to build and push the image to a resgistry with the following commands.

```bash
$ docker build -t image_name .
$ docker push image_name
```

In the [kubernetes](./kubernetes/) directory, there is the [deployment.yaml](./kubernetes/deployment.yaml) file which you just need to change line 21 to you image and add your environment variable as requested on line 34, then connect to your cluster and run the following command.

```bash
$ kubectl apply -f kubernetes/deployment.yaml
```

## License

Copyright 2022 João Pedro Poloni Ponce

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.