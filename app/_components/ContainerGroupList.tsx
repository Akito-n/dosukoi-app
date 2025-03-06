import { Button } from "@/components/ui/button";
import { ContainerGroup } from "../feature/container/types/container";
import DriveIcon from "../feature/icons/driveIcon";
import ArrowRightLineIcon from "../feature/icons/arrowRightLineIcon";
import StopIcon from "../feature/icons/stopIcon";
import { Dispatch, SetStateAction } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";


type Props = {
  groups: ContainerGroup[],
  selectedContainerName: string | null,
  setSelectedContainerName: Dispatch<SetStateAction<string | null>>
}
export const ContainerGroupList = ({
  groups,
  selectedContainerName,
  setSelectedContainerName
}: Props) => {
  const handleStopGroup = async (groupName: string) => {
    try{
      await invoke<string>('kill_group_containers', {groupName});
      toast(`Stopped containers in ${groupName}`)
    }catch (error) {
   
    toast(`Failed to stop containers in ${groupName}`);
  }

  }
  return (
     <ul>
       {groups.map((group) => (
         <li key={group.name} className="mb-4">
          <div className="flex items-center space-x-2">
             <h2 className="text-lg font-semibold">{group.name}</h2>
             <Button variant={'link'} 
              onClick={() => handleStopGroup(group.name)}
             ><StopIcon />
              </Button>
          </div>
          
           <ul className="ml-4">
             {group.containers.map((container) => (
               <li key={container} className="p-2 border-b flex items-center space-x-2 justify-between">
                <div className="flex items-center space-x-2">
                   <DriveIcon />
                <span>
                  {container}
                </span>
                </div>
                <Button className="cursor-pointer" onClick={() => setSelectedContainerName(container)}>
                  <ArrowRightLineIcon className="cursor-pointer" />
                </Button>
               </li>
             ))}
           </ul>
         </li>
       ))}
     </ul>
  );
};
