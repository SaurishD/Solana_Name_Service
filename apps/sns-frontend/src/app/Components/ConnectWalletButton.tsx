"use client";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";


export default function ConnectionButton() {
    const {publicKey} = useWallet();
    return <WalletMultiButton>{publicKey ? publicKey.toString().substring(0, 10)+" ...":"Connect"}</WalletMultiButton>
}