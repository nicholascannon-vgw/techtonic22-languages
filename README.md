# Tech-Tonic 2022 Workshop: Languages

Learn what makes programming languages unique and how to select the one that’s right for your project.

## Learning outcomes

-   Build and run software with a diverse set of programming languages
-   Understand the properties that make languages unique
-   Understand how the properties of a language change the artifact size, build time, run time and developer experience
-   Learn how to make an informed decision when selecting a language for your project

## Languages Covered

-   **Python / JavaScript**: high level, dynamically typed, interpreted, garbage collected scripting languages
-   **Java**: Compiled (but also interpreted 🧐), statically typed, garbaged collected programming language
-   **Go**: Natively compiled, statically typed, garbage collected programming language
-   **Rust / C**: Natively compiled, statically typed, _non_ garbage collected programming language

## How do I run my code?

All software in this workshop runs inside Docker!
This means you don't need to install any compilers or interpreters.

Use the `run.sh` script at the root of this repo to build and run the Docker containers for each language (`js`, `py`, `java`, `go`, `rust`, `c`):

```bash
➜  languages-workshop git:(master) ✗ ./run.sh js
[+] Building 0.1s (10/10) FINISHED
 => [internal] load build definition from Dockerfile                       0.0s
 => => transferring dockerfile: 37B                                        0.0s
 => [internal] load .dockerignore                                          0.0s
 => => transferring context: 34B                                           0.0s
 => [internal] load metadata for docker.io/library/node:16-alpine          0.0s
 => [internal] load build context                                          0.0s
 => => transferring context: 241B                                          0.0s
 => [1/5] FROM docker.io/library/node:16-alpine                            0.0s
 => CACHED [2/5] WORKDIR /usr/src/app                                      0.0s
 => CACHED [3/5] COPY package*.json .                                      0.0s
 => CACHED [4/5] RUN npm ci --production                                   0.0s
 => CACHED [5/5] COPY src/ src/                                            0.0s
 => exporting to image                                                     0.0s
 => => exporting layers                                                    0.0s
 => => writing image sha256:9d2d13c3be4199123a3be19cfce413e02da3b38302d1c  0.0s
 => => naming to docker.io/library/techtonic-js:lts                        0.0s

Use 'docker scan' to run Snyk tests against images to find vulnerabilities and learn how to fix them
Service started on port 8000...
```

Exit with `CTRL-C`.