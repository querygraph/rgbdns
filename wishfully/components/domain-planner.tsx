"use client";

import { FormEvent, useState } from "react";

type Plan = { domain: string; nameservers: string[]; records: string[]; verification: { owner: string; value: string } };

export function DomainPlanner() {
  const [domain, setDomain] = useState("example.com");
  const [kind, setKind] = useState<"A" | "ANAME">("ANAME");
  const [value, setValue] = useState("project.vercel.app");
  const [plan, setPlan] = useState<Plan | null>(null);
  const [error, setError] = useState("");

  async function submit(event: FormEvent) {
    event.preventDefault(); setError(""); setPlan(null);
    const response = await fetch("/api/v1/plan", { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ domain, destination: { type: kind, value }, includeWww: true }) });
    const body = await response.json();
    if (!response.ok) return setError("Use a complete domain and a valid destination.");
    setPlan(body);
  }

  return <section className="planner shell" id="planner"><div className="plannerCopy"><span className="overline">Try the compiler</span><h2>Plan a zone before signing up.</h2><p>This preview uses the same validation and tinydns record generator as the API. No DNS changes are made.</p></div><form onSubmit={submit}><label>Domain<input value={domain} onChange={(e) => setDomain(e.target.value)} autoCapitalize="none" /></label><div className="recordRow"><label>Destination<select value={kind} onChange={(e) => setKind(e.target.value as "A" | "ANAME")}><option value="ANAME">ANAME</option><option value="A">IPv4 address</option></select></label><label>Value<input value={value} onChange={(e) => setValue(e.target.value)} autoCapitalize="none" /></label></div><button className="button" type="submit">Build the plan</button>{error && <p className="formError">{error}</p>}{plan && <div className="planResult"><div><b>Delegate to</b><span>{plan.nameservers.join(" · ")}</span></div><pre>{plan.records.join("\n")}</pre><small>Ownership check: {plan.verification.owner}</small></div>}</form></section>;
}
