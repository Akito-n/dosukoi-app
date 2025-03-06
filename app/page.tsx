'use client'
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { ContainerGroup } from "./feature/container/types/container";
import { ContainerGroupList } from "./_components/ContainerGroupList";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";

export default function Home() {
  const [groups, setGroups] = useState<ContainerGroup[]>([]);
  const [selectedContainerName, setSelectedContainerName] = useState<string | null>(null);

  const handleAction = async(action: 'stop' | 'kill') => {
  if (!selectedContainerName) {
    return;
  }

  const switchCommand = action === 'stop' ? 'stop_container' : 'kill_container';

  console.log(`Invoking command: ${switchCommand} with container_name:`, selectedContainerName);

  try {
    await invoke<string>(switchCommand, { containerName: selectedContainerName });
    toast('success stop container');
    setSelectedContainerName(null);
  } catch (error) {
    console.error(`Failed to ${action} container:`, error);
    toast('failed stop container');
  }
};

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
        <div className={`flex ${selectedContainerName ? "flex-row gap-x-8" : "flex-col"}`}>
          
          <div className={selectedContainerName ? "w-2/3" : "w-full"}>
            <ContainerGroupList 
              groups={groups} 
              selectedContainerName={selectedContainerName} 
              setSelectedContainerName={setSelectedContainerName} 
            />
          </div>

        
          {selectedContainerName && (
            <div className="w-1/3 p-4  rounded">
              <Button 
                variant="destructive" 
                className="mb-2 w-full"
                onClick={() => setSelectedContainerName(null)}
              >
                Close
              </Button>
              <p className="font-semibold">{selectedContainerName}</p>
              <div className="mt-4 space-y-2">
                <Button 
                  variant="outline" 
                  className="w-full"
                  onClick={() => handleAction('stop')}
                >
                  dosukoi
                </Button>
                 <Button 
                  variant="outline" 
                  className="w-full"
                  onClick={() => handleAction('kill')}
                >
                  dosukoi kill
                </Button>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
