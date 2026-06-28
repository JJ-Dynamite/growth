"use client";
import { useState } from "react";
export default function Home() {
  const [teardowns] = useState([
    { id: "1", title: "Stripe Checkout Flow", company: "Stripe", summary: "How Stripe simplified payments", tags: ["checkout", "payments", "simplification"] },
    { id: "2", title: "Linear Onboarding", company: "Linear", summary: "The art of zero-config onboarding", tags: ["onboarding", "developer-tools"] },
  ]);
  const [selected, setSelected] = useState<any>(null);
  const [panels] = useState([
    { panel_number: 1, caption: "The landing page grabs attention immediately." },
    { panel_number: 2, caption: "The signup flow is frictionless." },
    { panel_number: 3, caption: "Users get value within seconds." },
  ]);
  return (
    <main className="min-h-screen bg-gradient-to-br from-red-900 via-black to-orange-900 text-white p-8">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-red-400 to-orange-400 bg-clip-text text-transparent">growth</h1>
        <p className="text-xl text-gray-300 mb-8">UX teardowns in comic form</p>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {teardowns.map((td) => (
            <div key={td.id} onClick={() => setSelected(td)}
              className="bg-white/10 backdrop-blur rounded-xl overflow-hidden hover:scale-105 transition cursor-pointer">
              <div className="aspect-[16/9] bg-white/20 flex items-center justify-center text-5xl">📚</div>
              <div className="p-4">
                <p className="font-bold text-lg">{td.title}</p>
                <p className="text-sm text-gray-400">{td.company}</p>
                <p className="text-sm mt-2">{td.summary}</p>
                <div className="flex gap-1 mt-2 flex-wrap">
                  {td.tags.map((t, i) => (
                    <span key={i} className="text-xs bg-red-600/30 px-2 py-0.5 rounded-full">{t}</span>
                  ))}
                </div>
              </div>
            </div>
          ))}
        </div>
        {selected && (
          <div className="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-8" onClick={() => setSelected(null)}>
            <div className="bg-white/10 backdrop-blur rounded-2xl p-8 max-w-3xl w-full max-h-[80vh] overflow-y-auto" onClick={(e) => e.stopPropagation()}>
              <h2 className="text-2xl font-bold mb-2">{selected.title}</h2>
              <p className="text-gray-400 mb-6">{selected.company} · {selected.summary}</p>
              <div className="space-y-4">
                {panels.map((p: any) => (
                  <div key={p.panel_number} className="bg-white/10 rounded-xl p-4">
                    <div className="aspect-video bg-white/20 rounded-lg mb-3 flex items-center justify-center text-2xl">🖼️ Panel {p.panel_number}</div>
                    <p className="text-sm">{p.caption}</p>
                  </div>
                ))}
              </div>
              <button onClick={() => setSelected(null)} className="mt-6 px-6 py-2 bg-red-600 rounded-full">Close</button>
            </div>
          </div>
        )}
      </div>
    </main>
  );
}