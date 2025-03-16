import styles from "./page.module.css";
import ConnectionButton from "./Components/ConnectWalletButton";

export default function Home() {
  return (
    <div className={styles.page}>
      <ConnectionButton/>
    </div>
  );
}
