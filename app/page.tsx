'use client'
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

interface ContainerGroup {
  name: string;
  containers: string[];
}

export default function Home() {
  const [groups, setGroups] = useState<ContainerGroup[]>([]);

  useEffect(() => {
    const fetchGroups = async () => {
      try {
        const result = await invoke<ContainerGroup[]>("get_grouped_containers");
        setGroups(result);
      } catch (error) {
        console.error("Failed to fetch containers:", error);
      }
    };

    fetchGroups();
    
    const unlisten = listen<ContainerGroup[]>("containers_updated", (event) => {
      setGroups(event.payload);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="p-6 max-w-2xl mx-auto">
      <h1 className="text-2xl font-bold mb-4">Docker Containers</h1>
      {groups.length === 0 ? (
        <p>No running containers.</p>
      ) : (
        <ul>
          {groups.map((group) => (
            <li key={group.name} className="mb-4">
              <h2 className="text-lg font-semibold">{group.name}</h2>
              <ul className="ml-4">
                {group.containers.map((container) => (
                  <li key={container} className="p-2 border-b">
                    {container}
                  </li>
                ))}
              </ul>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
