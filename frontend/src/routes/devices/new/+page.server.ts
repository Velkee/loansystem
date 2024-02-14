export async function load({ fetch }) {
	const fetched_categories = await fetch('http://localhost:8000/categories');
	const categories: {
		name: string;
	}[] = await fetched_categories.json();

	const fetched_people = await fetch('http://localhost:8000/people');
	const people: {
		id: string;
		name: string;
		location: string;
		department: string;
		building: string;
	}[] = await fetched_people.json();

	return { categories, people };
}
