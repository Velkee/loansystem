import type { status } from '$lib';

export async function load({ fetch }) {
    const fetched_devices = await fetch('http://localhost:8000/devices')
    const devices: {
        id: string;
        serial_number: string;
        category: string;
        status: status;
        person: string;
        location: string;
        department: string;
        building: string;
        room: string;
    }[] = await fetched_devices.json();

    return { devices }
}