import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  metadataBase: new URL("https://wishful.ly"),
  title: "Wishfully — calm, authoritative DNS",
  description:
    "Host domains on rgbdns with auditable zone publishing and scoped Let's Encrypt automation.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
