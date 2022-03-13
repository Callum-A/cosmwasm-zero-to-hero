# Part Seventeen - Front End Template

Alright to bootstrap our front end setup, we're going to be using this [Repo](https://github.com/CosmosContracts/starter-kit) provided by Juno.

If you read the README.md here are the commands we need to run:

```
npx create-next-app -e https://github.com/cosmoscontracts/starter-kit my-cosmwasm-dapp
# or
yarn create next-app -e https://github.com/cosmoscontracts/starter-kit my-cosmwasm-dapp
```

I'm going to call my project cw-starter-front. For the front end. It will now also be in `code` directory for the tutorial.

I choose to use `npm` for my dependencies but it's just a personal preference.

Here's the exact command I decided to run:

```
npx create-next-app -e https://github.com/cosmoscontracts/starter-kit cw-starter-front
```

Now to test it's all setup correctly lets run our local chain. Change directory our contract root or have a second terminal open and run:

```
./scripts/deploy_local.sh
```

This will setup our local chain and have it running in a background container.

Now go to the root of the `cw-starter-front` and run the following:

```
cp .env.example .env.local
```

This sets it up for our local environment.

Then run:

```
npm run dev
```

The output should look something like:

```

> next-cosmwasm-keplr-starter@0.1.0 dev
> rm -r dist || true && next dev

rm: dist: No such file or directory
ready - started server on 0.0.0.0:3000, url: http://localhost:3000
info  - Loaded env from /Users/callumanderson/Documents/Writing/cosmwasm-zero-to-hero/17 - Front End Template/code/cw-starter-front/.env.local

warn - You have enabled the JIT engine which is currently in preview.
warn - Preview features are not covered by semver, may introduce breaking changes, and can change at any time.

🌼 DaisyUI components 1.14.0  https://github.com/saadeghi/daisyui
  ✔︎ Including:  base, components, themes[2], utilities

event - compiled client and server successfully in 4.8s (560 modules)
```

You should be met with a connect wallet page, make sure under your Keplr you have selected the unsafe seed user we added last tutorial.

Accept any pop ups it should add our local chain to Keplr.

To check open your Keplr and open the networks selection menu, it should be added under the name "Juno Local Test" and you should have 1000 JUNOX.

The page should also display your wallet address once connected.

All good? Perfect in the next part we'll start stripping out this boilerplate and get started developing.
