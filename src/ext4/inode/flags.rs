use bitflags::bitflags;

bitflags! {
  pub struct Flags: u32
  {
    /// This file requires secure deletion (EXT4_SECRM_FL). (not implemented)
    const SECURE_DELETION = 0x1;
    /// This file should be preserved, should undeletion be desired
    /// (EXT4_UNRM_FL). (not implemented)
    const SHOULD_PRESERVE = 0x2;
    /// File is compressed (EXT4_COMPR_FL). (not really implemented)
    const COMPRESSED = 0x4;
    /// All writes to the file must be synchronous (EXT4_SYNC_FL).
    const SYNCHRONOUS = 0x8;
    /// File is immutable (EXT4_IMMUTABLE_FL).
    const IMMUTABLE = 0x10;
    /// File can only be appended (EXT4_APPEND_FL).
    const ONLY_APPEND = 0x20;
    /// The dump(1) utility should not dump this file (EXT4_NODUMP_FL).
    const NODUMP = 0x40;
    /// Do not update access time (EXT4_NOATIME_FL).
    const NOATIME = 0x80;
    /// Dirty compressed file (EXT4_DIRTY_FL). (not used)
    const DIRTY_COMPRESSED = 0x100;
    /// File has one or more compressed clusters (EXT4_COMPRBLK_FL). (not used)
    const COMPRESSED_CLUSTERS = 0x200;
    /// Do not compress file (EXT4_NOCOMPR_FL). (not used)
    const NOCOMPRESSED = 0x400;
    /// Encrypted inode (EXT4_ENCRYPT_FL). This bit value previously was
    /// EXT4_ECOMPR_FL (compression error), which was never used.
    const ENCRYPTED = 0x800;
    /// Directory has hashed indexes (EXT4_INDEX_FL).
    const HASHED_INDEXES = 0x1000;
    /// AFS magic directory (EXT4_IMAGIC_FL).
    const MAGIC_DIR = 0x2000;
    /// File data must always be written through the journal
    /// (EXT4_JOURNAL_DATA_FL).
    const JOURNAL_DATA = 0x4000;
    /// File tail should not be merged (EXT4_NOTAIL_FL). (not used by ext4)
    const NOTAIL = 0x8000;
    /// All directory entry data should be written synchronously (see dirsync)
    /// (EXT4_DIRSYNC_FL).
    const DIRSYNCHRONOUS = 0x10000;
    /// Top of directory hierarchy (EXT4_TOPDIR_FL).
    const TOPDIR = 0x20000;
    /// This is a huge file (EXT4_HUGE_FILE_FL).
    const HUGE_FILE = 0x40000;
    /// Inode uses extents (EXT4_EXTENTS_FL).
    const EXTENTS = 0x80000;
    /// Verity protected file (EXT4_VERITY_FL).
    const VERITY = 0x100000;
    /// Inode stores a large extended attribute value in its data blocks
    /// (EXT4_EA_INODE_FL).
    const EA_INODE = 0x200000;
    /// This file has blocks allocated past EOF (EXT4_EOFBLOCKS_FL). (deprecated)
    const EOFBLOCKS = 0x400000;
    /// Inode is a snapshot (EXT4_SNAPFILE_FL). (not in mainline)
    const SNAPFILE = 0x01000000;
    /// Snapshot is being deleted (EXT4_SNAPFILE_DELETED_FL). (not in mainline)
    const SNAPFILE_DELETED = 0x04000000;
    /// Snapshot shrink has completed (EXT4_SNAPFILE_SHRUNK_FL). (not in mainline)
    const SNAPFILE_SHRUNK = 0x08000000;
    /// Inode has inline data (EXT4_INLINE_DATA_FL).
    const INLINE = 0x10000000;
    /// Create children with the same project ID (EXT4_PROJINHERIT_FL).
    const PROJECT_ID_INHERIT = 0x20000000;
    /// Reserved for ext4 library (EXT4_RESERVED_FL).
    const RESERVED = 0x80000000;

    // Aggregate flags:
    /// User-visible flags.
    const USER_VISIBLE = 0x705BDFFF;
    /// User-modifiable flags. Note that while EXT4_JOURNAL_DATA_FL and
    /// EXT4_EXTENTS_FL can be set with setattr, they are not in the kernel’s
    /// EXT4_FL_USER_MODIFIABLE mask, since it needs to handle the setting of
    /// these flags in a special manner and they are masked out of the set of
    /// flags that are saved directly to i_flags.
    const USER_MODIFIABLE = 0x604BC0FF;
  }
}
