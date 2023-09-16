import { css } from "../../styled-system/css";
import { center } from "../../styled-system/patterns";

export default function Home() {
  return (
    <main className={center()}>
      <h1 className={css({ fontSize: "x-large" })}>Hello World!</h1>
    </main>
  );
}
