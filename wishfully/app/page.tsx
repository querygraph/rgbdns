import { DomainPlanner } from "@/components/domain-planner";

const domains = [
  { name: "fieldnotes.es", state: "Serving", serial: "2026072902", tls: "Ready" },
  { name: "foto.gs", state: "Serving", serial: "2026073101", tls: "DNS only" },
  { name: "chiefscientist.org", state: "Checking delegation", serial: "2026073101", tls: "Waiting" },
];

export default function Home() {
  return (
    <main>
      <nav className="nav shell">
        <a className="brand" href="#top" aria-label="Wishfully home"><span>W</span> Wishfully</a>
        <div className="navLinks"><a href="#how">How it works</a><a href="#pricing">Pricing</a><a href="#cli">CLI</a></div>
        <a className="button buttonSmall" href="#planner">Add a domain</a>
      </nav>

      <section className="hero shell" id="top">
        <div className="eyebrow"><i /> Powered by rgbdns</div>
        <h1>Authoritative DNS,<br /><em>without the wishful thinking.</em></h1>
        <p className="lede">A quiet control plane for domains on <code>a.ns.cron.sh</code> and <code>b.ns.cron.sh</code>—with reviewable changes, atomic delivery, and scoped Let&apos;s Encrypt automation.</p>
        <div className="heroActions"><a className="button" href="#planner">Plan your first domain</a><a className="textLink" href="#how">See the publishing path <span>→</span></a></div>
        <div className="proof"><div><strong>2</strong><span>independent authorities</span></div><div><strong>1</strong><span>consolidated source</span></div><div><strong>0</strong><span>private keys uploaded</span></div></div>
      </section>

      <section className="dashboard shell">
        <div className="windowBar"><span /><span /><span /><small>wishful.ly / domains</small></div>
        <div className="dashboardBody">
          <aside><div className="sideBrand">W</div><a className="active">Domains</a><a>Certificates</a><a>Deployments</a><a>API keys</a><div className="sideBottom"><b>Alexy&apos;s studio</b><span>Maker plan</span></div></aside>
          <div className="domainList"><header><div><span className="overline">Your domains</span><h2>Everything is answering.</h2></div><button>+ Add domain</button></header>
            {domains.map((domain) => <article key={domain.name}><div className={`status ${domain.state === "Serving" ? "good" : "pending"}`} /><div className="domainName"><b>{domain.name}</b><span>{domain.state}</span></div><div><small>SOA serial</small><b>{domain.serial}</b></div><div><small>Certificate</small><b>{domain.tls}</b></div><span className="chevron">›</span></article>)}
          </div>
        </div>
      </section>

      <section className="how shell" id="how"><div className="sectionIntro"><span className="overline">One dependable path</span><h2>From intent to both nameservers.</h2><p>Wishfully turns each customer domain into a small manifest. The compiler combines every approved manifest into the same two files rgbdns already deploys today.</p></div>
        <div className="steps"><article><span>01</span><h3>Prove ownership</h3><p>Add one TXT value at the current provider. Wishfully verifies it before accepting the zone.</p></article><article><span>02</span><h3>Review the plan</h3><p>Choose an address or ANAME target. A pull request shows the exact zone records and serial.</p></article><article><span>03</span><h3>Delegate</h3><p>After both authorities answer identically, point the registrar at a.ns.cron.sh and b.ns.cron.sh.</p></article><article><span>04</span><h3>Automate TLS</h3><p>Download a scoped RFC 2136 credential. Certbot presents and cleans up DNS-01 without exposing your key.</p></article></div>
      </section>

      <DomainPlanner />

      <section className="pricing shell" id="pricing"><div className="sectionIntro"><span className="overline">Pricing</span><h2>Small enough to understand.</h2></div><div className="plans">
        <article><h3>Seed</h3><div className="price">$0 <span>/ month</span></div><p>For a personal domain that deserves real secondary DNS.</p><ul><li>1 zone, 25 records</li><li>A, CNAME, MX, TXT</li><li>Auditable zone publishing</li><li>Community support</li></ul><a href="#planner">Start with Seed</a></article>
        <article className="featured"><div className="popular">Most useful</div><h3>Maker</h3><div className="price">$9 <span>/ month</span></div><p>For portfolios, publications, and independent products.</p><ul><li>10 zones, unlimited records</li><li>ANAME apex flattening</li><li>Scoped Certbot credentials</li><li>Change history and rollback</li><li>Email support</li></ul><a href="#planner">Choose Maker</a></article>
        <article><h3>Studio</h3><div className="price">$39 <span>/ month</span></div><p>For teams managing a serious domain portfolio.</p><ul><li>100 zones</li><li>Team roles and audit log</li><li>Bulk import and API</li><li>Delegated ACME validation zones</li><li>Priority support</li></ul><a href="mailto:hello@wishful.ly">Talk to us</a></article></div>
      </section>

      <section className="cli shell" id="cli"><div><span className="overline">The same control plane, from your shell</span><h2>Automate without bypassing review.</h2><p>The CLI submits the same domain manifests as the web app and waits for authoritative convergence.</p></div><pre><code><span>$</span> npx wishfully domains add example.com --aname site.vercel.app{`\n`}<i>✓ ownership challenge created</i>{`\n`}<span>$</span> wishfully domains verify example.com{`\n`}<i>✓ ready to delegate to a.ns.cron.sh, b.ns.cron.sh</i>{`\n`}<span>$</span> wishfully certbot run example.com{`\n`}<i>✓ certificate saved locally; private key never uploaded</i></code></pre></section>

      <footer className="shell"><a className="brand" href="#top"><span>W</span> Wishfully</a><p>Authoritative DNS with fewer surprises.</p><div><a href="https://github.com/querygraph/rgbdns">rgbdns</a><a href="mailto:hello@wishful.ly">Contact</a></div></footer>
    </main>
  );
}
