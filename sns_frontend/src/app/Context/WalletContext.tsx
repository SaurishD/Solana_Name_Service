"use client";

import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter, SolflareWalletAdapter } from "@solana/wallet-adapter-wallets";
import { clusterApiUrl } from "@solana/web3.js";
import { FC, ReactNode, useMemo } from "react";

import "@solana/wallet-adapter-react-ui/styles.css";

const WalletContext: FC<{ children: ReactNode }> = ({children}) => {

    const endpoint = clusterApiUrl("devnet")
    const wallets = useMemo(() => [new PhantomWalletAdapter(), new SolflareWalletAdapter()],[])

    return (
        <ConnectionProvider endpoint={endpoint}>
           <WalletProvider wallets={wallets} autoConnect>
             <WalletModalProvider>
                {children}
             </WalletModalProvider>
           </WalletProvider>
        </ConnectionProvider>
       ); 
}

export default WalletContext;