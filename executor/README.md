# Prerequisites

Currently, executor is dependent of eigen-prover repo. Make sure you have eigen-prover repo locally,
and create soft links under this directiry to SM and starksj directory.
```bash
cd $YOUR_DIRECTORY
git clone git@github.com:0xEigenLabs/eigen-zkvm.git

# under eigen-prover/executor directory
ln -s $YOUR_DIRECTORY/eigen-zkvm/SM .
ln -s $YOUR_DIRECTORY/eigen-zkvm/starkjs .

npm i
```


# Steps to test the Fibonacci example

```bash
cp .example .env
npm run testFib
```

When the test is completed, kill the server

```bash
npm run killServer
```
