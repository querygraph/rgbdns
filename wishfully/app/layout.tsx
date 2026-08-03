import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  metadataBase: new URL("https://wishful.ly"),
  title: "Wishfully — calm, authoritative DNS",
  description:
    "Host domains on rgbdns with auditable zone publishing and scoped Let's Encrypt automation.",
  openGraph: {
    title: "Wishfully — calm, authoritative DNS",
    description: "Authoritative DNS, without the wishful thinking.",
    url: "https://wishful.ly",
    siteName: "Wishfully",
    images: [{ url: "/og.png", width: 1200, height: 630 }],
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    title: "Wishfully — calm, authoritative DNS",
    description: "Authoritative DNS, without the wishful thinking.",
    images: ["/og.png"],
  },
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
