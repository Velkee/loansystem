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

        switch (device.status) {
            case status.InUse: device.status = "In use"
            case status.InStock: device.status = "In stock"
            case status.HomeOffice: device.status = "Used in home office"
            case status.Temporary: device.status = "Temporary"
            case status.Lost: device.status = "Lost"
            case status.ToBeRepaired: device.status = "To be repaired"
            case status.Return: device.status = "Return supplier"
            case status.Sold: device.status = "Sold"
            case status.Stolen: device.status = "Stolen"
            case status.AssignedLoanOffice: device.status = "Assigned loan office"
            case status.Loaned: device.status = "Loaned out"
            case status.Discarded: device.status = "Discarded"
            case status.Other: device.status = "Other"
        }

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