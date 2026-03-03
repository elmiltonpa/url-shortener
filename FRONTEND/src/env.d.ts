/// <reference types="astro/client" />
declare namespace App {
  interface Locals {
    user: {
      id: string;
      email: string;
      username: string;
    } | null;
    token: string | null;
  }
}
