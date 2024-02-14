import { status } from '$lib';

export async function load({ fetch }) {
    const fetched_devices = await fetch('http://localhost:8000/devices')
    const devices: {
        id: string;
        serial_number: string | null;
        category: string | null;
        status: status | string;
        person: string | null;
        location: string | null;
        department: string | null;
        building: string | null;
        room: string | null;
    }[] = await fetched_devices.json();

    devices.forEach( (device) => {
        if (device.serial_number === null) {
            device.serial_number = ""
        }
        
        if (device.category === null) {
            device.category = "None"
        }

        if (device.person === null) {
            device.person = ""
        }

        if (device.location === null) {
            device.location = ""
        }

        if (device.department === null) {
            device.department = ""
        }

        if (device.building === null) {
            device.building = ""
        }

        if (device.room === null) {
            device.room = ""
        }
    })

    return { devices }
}