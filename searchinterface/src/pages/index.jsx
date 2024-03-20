import Home from './home'; // adjust the path according to your project structure

export const metadata = {
 title: 'Edu-cate',
 description: 'Search engine and interface for educational content.',
};

export default function RootLayout() {
 return (
   <div style={{display: 'flex', height: '100vh', overflowY:'auto', overflowX:'hidden'}}>
     <Home />
   </div>
 );
}


