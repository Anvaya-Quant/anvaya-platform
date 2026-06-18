'use client';

import { useState, useEffect } from 'react';
import { getCore } from '@/lib/anvaya-core';

export function useCoreReady() {
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getCore()
      .then(() => setReady(true))
      .catch(err => setError(err.message || String(err)));
  }, []);

  return { ready, error };
}
