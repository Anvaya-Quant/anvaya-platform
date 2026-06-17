import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'ANVAYA Studio',
  description: 'Visual Quantum Circuit IDE',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="bg-gray-950 text-gray-100">{children}</body>
    </html>
  );
}
