# Part Sixteen - Running a Local Chain

Alright in order to test our front-end dApp we're going to need to deploy it to a chain, here are our options:

1. Run a chain locally.
2. Use a test net like uni-2 etc.

I choose to use running a chain locally as it allows more flexibility and is generally faster.

So how do we do this well we're doing to need to pull in some files. (Credit to the-frey AKA needlecast for this).

I'm not going to go over the specifics just the requirements, so you're going to need have `docker` installed and usable via the CLI. There are plenty of tutorials for getting it setup so I'm sure you can find one for your OS.

Next look in `code/cw-starter` you should notice a new `docker` folder, copy that over to your project.

You should also notice a `scripts` folder containing `deploy_local.sh`, also copy that over.

If you want to cover the functionality it effectively runs a chain in the background and uploads the contract for you outputting all variables we will need for the front end in a format like:

```
 ------------------------
Config Variables

NEXT_PUBLIC_CW_STARTER_CODE_ID=1
NEXT_PUBLIC_CW_STARTER_ADDRESS=juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8
0
```

We will need these for our front end. Also quick PSA always call the script from the root, the command should look like this when calling the script:

```
./scripts/deploy_local.sh
```

Also to save us time in the next tutorial add the following mneumonic to your keplr as "UNSAFE SEED USER" (WHATEVER YOU DO NEVER HOLD REAL FUNDS WITH THIS).

The mneumonic (taken from Juno docs):

```
clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose
```

This will purely be used for our local net which we can now run.

In the next tutorial we will get out front end repo setup.
