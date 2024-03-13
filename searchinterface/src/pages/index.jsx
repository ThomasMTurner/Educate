import type { Metadata } from 'next';
import Home from './home'; // adjust the path according to your project structure

export const metadata: Metadata = {
 title: 'Edu-cate',
 description: 'Search engine and interface for educational content.',
};

export default function RootLayout() {
 return (
   <div>
     <Home />
   </div>
 );
}


