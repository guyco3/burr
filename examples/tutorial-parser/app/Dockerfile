FROM node:22-slim

# Copy pre-compiled virtualizer CLI if we are building locally,
# but for the tutorial, we just use the pre-built CLI from GitHub or compile it.
# Actually, the user's tutorial says `wrdn install`, so we need `wrdn` inside the container.
# We will download the latest wrdn release or copy it from the host.

WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm install

# We clear the environment to just what we need to minimize "deny" log spam
ENV PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
ENV DEBUG_MODE="1"

COPY . .

CMD ["npm", "start"]
