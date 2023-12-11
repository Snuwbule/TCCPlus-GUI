<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { parse } from "plist";
  // variable setup
  let bundleId = "";
  let result = "Drag and Drop your app here!";
  let doableActions = [
    { id: 1, text: `add` },
    { id: 2, text: `remove` },
  ];
  let doableServices = [
    { id: 1, text: `All` },
    { id: 2, text: `Accessibility` },
    { id: 3, text: `AddressBook` },
    { id: 4, text: `AppleEvents` },
    { id: 5, text: `Calendar` },
    { id: 6, text: `Camera` },
    { id: 7, text: `ContactsFull` },
    { id: 8, text: `ContactsLimited` },
    { id: 9, text: `DeveloperTool` },
    { id: 10, text: `Facebook` },
    { id: 11, text: `LinkedIn` },
    { id: 12, text: `ListenEvent` },
    { id: 13, text: `Liverpool` },
    { id: 14, text: `Location` },
    { id: 15, text: `MediaLibrary` },
    { id: 16, text: `Microphone` },
    { id: 17, text: `Motion` },
    { id: 18, text: `Photos` },
    { id: 19, text: `PhotosAdd` },
    { id: 20, text: `PostEvent` },
    { id: 21, text: `Reminders` },
    { id: 22, text: `ScreenCapture` },
    { id: 23, text: `ShareKit` },
    { id: 24, text: `SinaWeibo` },
    { id: 25, text: `Siri` },
    { id: 26, text: `SpeechRecognition` },
    { id: 27, text: `SystemPolicyAllFiles` },
    { id: 28, text: `SystemPolicyDesktopFolder` },
    { id: 29, text: `SystemPolicyDeveloperFiles` },
    { id: 30, text: `SystemPolicyDocumentsFolder` },
    { id: 31, text: `SystemPolicyDownloadsFolder` },
    { id: 32, text: `SystemPolicyNetworkVolumes` },
    { id: 33, text: `SystemPolicyRemovableVolumes` },
    { id: 34, text: `SystemPolicySysAdminFiles` },
    { id: 35, text: `TencentWeibo` },
    { id: 36, text: `Twitter` },
    { id: 37, text: `Ubiquity` },
    { id: 38, text: `Willow` },
  ];

  let action;
  let service;

  // looking for bundle id when get file drop

  listen("tauri://file-drop", (event) => {
    var payload = event.payload[0];
    console.log(payload);
    invoke("find_bundle_id", { payload }).then(async (result) => {
      var plistFile = parse(result);
      bundleId = plistFile.CFBundleIdentifier;
    });
  });

  // calling tccplus

  const sumbitFeature = async () => {
    console.log(action.text, service.text, bundleId);
    result = await invoke("send", { action, service, bundleId });
  };
</script>

<main>
  <div
    class="w-screen h-screen flex flex-col gap-4 bg-[#282828] justify-center items-center"
  >
    <div>
      <form on:submit|preventDefault={sumbitFeature} class="flex flex-col gap-4">
        <input class="text-white px-2 bg-[#232323] rounded-md appearance-none" bind:value={bundleId} placeholder="ID" />

        <select class="appearance-none px-2 bg-[#232323] rounded-md text-white" bind:value={action}>
          {#each doableActions as acts}
            <option value={acts.text}>
              {acts.text}
            </option>
          {/each}
        </select> 

        <select class="text-white px-2 appearance-none bg-[#232323]" bind:value={service}>
          {#each doableServices as serv}
            <option value={serv.text}>
              {serv.text}
            </option>
          {/each}
        </select> 

        <button class="text-white appearance-none disabled:bg-[#232323] active:bg-blue-800 bg-blue-500 rounded-md" type="submit" disabled={!service || !bundleId || !action}>Send</button>
      </form>
    </div>
    <p class="text-white" >{result}</p>
  </div>
</main>