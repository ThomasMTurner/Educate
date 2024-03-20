export default function History({searches}) {
    return (
        <div>
            {searches.map((search, index) => (
                <p key={index}>{search}</p>
            ))}
        </div>
    )
}
