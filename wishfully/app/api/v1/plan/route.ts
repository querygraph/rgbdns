import { NextResponse } from "next/server";
import { domainRequest, planDomain } from "@/lib/domain";

export async function POST(request: Request) {
  const parsed = domainRequest.safeParse(await request.json().catch(() => null));
  if (!parsed.success) {
    return NextResponse.json(
      { error: "Invalid domain plan", details: parsed.error.flatten() },
      { status: 422 },
    );
  }
  return NextResponse.json(planDomain(parsed.data));
}
