![GitHub contributors](https://img.shields.io/github/contributors/joaopedropp/uRedis?style=flat-square)
[![GitHub issues](https://img.shields.io/github/issues/joaopedropp/uRedis?style=flat-square)](https://github.com/joaopedropp/uRedis/issues)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/JoaoPedroPP/uRedis/blob/main/LICENSE)
# µRedis - A Rust Microservice to recod and read data in Redis

Table of contents
- [About the Project](#about-the-project)
    - [Built with](#built-with)
- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Run](#run)
- [API docs](#api-docs)
    - [Save](#save)
    - [Cache](#cache)
    - [Read](#read)
    - [Delete](#delete)
- [Deployment](#deployment)
    - [Docker](#1-docker)
    - [Kubernetes](#2-kubernetes)
- [License](#license)

## About the Project

µRedis is a microservice that implement a connection pool on Redis Database as well as the operations to save, cache, read and delete data. This project was created in order to speed up development phase by deliving a fast and secure access to a connection pool through REST API services. Instead of creating a db connection inside the application, developers can connect their application through µRedis and their APIs to save, cache, read and delete data.

### Built with

This project was built with a some library listed on [Cargo.toml](Cargo.toml) file, however the main components are listed below.

1. [Rust](https://www.rust-lang.org)
2. [Actix-Web](https://actix.rs/)
3. [Redis-rs](https://github.com/redis-rs/redis-rs)
4. [Deadpool](https://github.com/bikeshedder/deadpool)

## Getting Started

To run this project locally you will need the prerequisites listed below. 

### Prerequisites
* Rust >= 1.59.20
* Cargo >= 1.59.20
### Run
```
cargo run
```

## API docs
### Save

* **URL**

  /api/save

* **Method:**

  `POST`

* **Data Params**

  ```json
    {
        "key": "REDIS_KEY",
        "payload": "String|JSON"
    }
  ```

* **Success Response:**

  * **Code:** 200 <br />
    **Content:** `{ "code": 201, "msg": "Success" }`

### Cache
* **URL**

  /api/cache

* **Method:**

  `POST`

* **Data Params**

  ```json
    {
        "key": "REDIS_KEY",
        "ttl": 10, // int seconds
        "payload": "String|JSON"
    }
  ```

* **Success Response:**

  * **Code:** 200 <br />
    **Content:** `{ "code": 201, "msg": "Success" }`
### Read
* **URL**

  /api/read

* **Method:**

  `POST`

* **Data Params**

  ```json
    {
        "key": "REDIS_KEY"
    }
  ```

* **Success Response:**

  * **Code:** 200 <br />
    **JSON Content:** `{
	"code": 200,
	"msg": "Success",
	"payload": {
		"JSON": {
			"a": "a string"
		}
	}
}`

    **String Content:** `{
	"code": 200,
	"msg": "Success",
	"payload": {
		"Simple": "a string"
	}
}`
### Delete
* **URL**

  /api/delete/:key

* **Method:**

  `GET`

* **Success Response:**

  * **Code:** 200 <br />
    **Content:** `{ "code": 200, "msg": "Deleted" }`
## Deployment

### Docker

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

### Kubernetes

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