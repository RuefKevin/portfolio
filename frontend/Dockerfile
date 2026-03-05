#========================================
# Stage 1: Builder
#========================================
FROM node:20-slim AS builder

WORKDIR /app

COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build

#========================================
# Stage 2: Runtime
#========================================
FROM node:20-slim

RUN useradd -m -s /bin/bash appuser
USER appuser
WORKDIR /home/appuser

COPY --from=builder /app/build ./build
COPY --from=builder /app/package*.json ./

ENV NODE_ENV=production
ENV PORT=3000

EXPOSE 3000

CMD ["node", "build"]