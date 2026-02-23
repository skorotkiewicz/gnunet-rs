import { useCallback, useEffect, useRef, useState } from "react";
import type { ClientMessage, ServerMessage } from "../types";

type MessageHandler = (msg: ServerMessage) => void;

export function useWebSocket(url: string) {
	const wsRef = useRef<WebSocket | null>(null);
	const [connected, setConnected] = useState(false);
	const handlersRef = useRef<Set<MessageHandler>>(new Set());
	const reconnectTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(
		null,
	);
	const urlRef = useRef(url);

	useEffect(() => {
		urlRef.current = url;
	}, [url]);

	const cleanup = useCallback(() => {
		if (reconnectTimeoutRef.current) {
			clearTimeout(reconnectTimeoutRef.current);
			reconnectTimeoutRef.current = null;
		}
		if (wsRef.current) {
			wsRef.current.onclose = null;
			wsRef.current.onerror = null;
			wsRef.current.onopen = null;
			wsRef.current.onmessage = null;
			wsRef.current.close();
			wsRef.current = null;
		}
	}, []);

	const connect = useCallback(() => {
		cleanup();

		const ws = new WebSocket(urlRef.current);

		ws.onopen = () => {
			setConnected(true);
		};

		ws.onclose = (event) => {
			setConnected(false);
			wsRef.current = null;
			if (event.code !== 1000) {
				reconnectTimeoutRef.current = setTimeout(connect, 2000);
			}
		};

		ws.onerror = () => {
			ws.close();
		};

		ws.onmessage = (event) => {
			try {
				const msg: ServerMessage = JSON.parse(event.data);
				handlersRef.current.forEach((h) => {
					h(msg);
				});
			} catch {
				console.error("Failed to parse message");
			}
		};

		wsRef.current = ws;
	}, [cleanup]);

	const send = useCallback((msg: ClientMessage) => {
		if (wsRef.current?.readyState === WebSocket.OPEN) {
			wsRef.current.send(JSON.stringify(msg));
			return true;
		}
		return false;
	}, []);

	const subscribe = useCallback((handler: MessageHandler) => {
		handlersRef.current.add(handler);
		return () => {
			handlersRef.current.delete(handler);
		};
	}, []);

	useEffect(() => {
		connect();
		return cleanup;
	}, [connect, cleanup]);

	return { connected, send, subscribe };
}
