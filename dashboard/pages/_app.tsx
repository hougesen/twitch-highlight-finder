import '../styles/globals.css';

import { AppProps } from 'next/app';

import TopNavigation from '../components/TopNavigation';

export default function App({ Component, pageProps }: AppProps) {
    return (
        <div className="flex gap-8 container mx-auto  flex-col">
            <TopNavigation />

            <div className="">
                <Component {...pageProps} />
            </div>
        </div>
    );
}
