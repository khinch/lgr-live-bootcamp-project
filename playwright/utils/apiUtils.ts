import { expect, APIRequestContext } from "@playwright/test";
import { getEnv } from "./envUtils";

const postmarkServerToken: string = getEnv("POSTMARK_AUTH_TOKEN");

async function fetchLatestMessageId(
  request: APIRequestContext,
  recipientEmail: string,
  maxTimeoutMs: number
): Promise<string> {
  const startTime = Date.now();
  while (Date.now() - startTime < maxTimeoutMs) {
    const messagesResponse = await request.get(
      "https://api.postmarkapp.com/messages/outbound",
      {
        params: { offset: 0, count: 500, recipient: recipientEmail },
        headers: {
          Accept: "application/json",
          "X-Postmark-Server-Token": postmarkServerToken,
        },
      }
    );
    expect(messagesResponse.ok()).toBeTruthy();

    const messages = await messagesResponse.json();
    if (messages.TotalCount > 0) {
      if (messages.TotalCount > 1) {
        console.warn(
          "More than one email for this recipient. 2FA code could be out-of-date."
        );
      }
      return messages.Messages[0].MessageID;
    }
    console.log("No messages yet. Waiting 1s ...");
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  throw new Error("Timeout: No message received after " + maxTimeoutMs + "ms.");
}

async function fetchMessageTextBody(
  request: APIRequestContext,
  messageId: string,
  maxTimeoutMs: number
): Promise<string> {
  const startTime = Date.now();
  while (Date.now() - startTime < maxTimeoutMs) {
    const messageResponse = await request.get(
      `https://api.postmarkapp.com/messages/outbound/${messageId}/details`,
      {
        headers: {
          Accept: "application/json",
          "X-Postmark-Server-Token": postmarkServerToken,
        },
      }
    );
    if (messageResponse.ok()) {
      const message = await messageResponse.json();
      if (message.TextBody) {
        return message.TextBody;
      }
    }
    console.log("No message details yet. Waiting 1s ...");
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  throw new Error(
    "Timeout: No message details received after " + maxTimeoutMs + "ms."
  );
}

export async function retrieveTwoFaCode(
  request: APIRequestContext,
  recipientEmail: string
): Promise<string> {
  const messageId = await fetchLatestMessageId(request, recipientEmail, 30000); // 30 sec timeout
  expect(messageId).toBeDefined();

  const textBody = await fetchMessageTextBody(request, messageId, 30000); // 30 sec timeout
  expect(textBody).toBeDefined();

  return textBody;
}
