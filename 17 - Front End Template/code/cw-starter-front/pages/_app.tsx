import "styles/globals.css";
import type { AppProps } from "next/app";
import Layout from "components/Layout";
import { SigningCosmWasmProvider } from "contexts/cosmwasm";

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <SigningCosmWasmProvider>
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </SigningCosmWasmProvider>
  );
}
export default MyApp;
