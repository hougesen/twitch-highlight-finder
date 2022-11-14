/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    swcMinify: true,
    images: {
        domains: ['static-cdn.jtvnw.net'],
    },
    redirects: async () => [
        {
            source: '/',
            destination: '/channels',
            permanent: true,
        },
    ],
};

module.exports = nextConfig;
