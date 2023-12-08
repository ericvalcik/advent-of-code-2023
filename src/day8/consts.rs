pub const EXAMPLE1: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

pub const EXAMPLE2: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

pub const EXAMPLE3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

pub const INPUT: &str = "\
LLRLRRRLLLRLRRLRRRLRLRRLRLRLRRRLRRRLRLRLRRLLRRRLRRLRRLLRLRRRLRLRLLRRRLLRRRLRLRRRLRRRLRRRLLLRRRLRRLRRLRLRRLRLRRRLRLRRLRLRLRRRLRLLLRRRLLLRLRRRLRLRRLRLRLRLRRLRRLRRLRLRRRLRRRLRRLRRRLRRLRRLRRRLLRLRRLLLRRLRRLRLRLLLRRLRRLRRRLRRLLRLRRRLRRRLRRLRRLRLRRLRLRRRLRRLRRRLLRRRLRLRLLLRRRLLLRRLLRRLRLRRLRLLLRRRR

BNR = (DLB, QNM)
CTN = (NVJ, DRS)
VHS = (XDL, LVL)
MLQ = (VXK, CJK)
CNX = (CTN, DRK)
MVD = (SRT, SRT)
LLM = (VKX, VTT)
GDS = (PJT, DBD)
QFF = (BQR, HCT)
QNP = (NXM, MRQ)
SKT = (GGD, BXR)
KND = (JKT, VKS)
NSJ = (VLH, HMK)
TKV = (SMR, BVG)
STR = (KFR, NQK)
VDL = (PRB, KLG)
DBN = (NPF, VDL)
PCQ = (CNL, QBX)
PQS = (KKJ, GNV)
JQM = (DBD, PJT)
FHL = (SSL, PLN)
KVT = (KBG, SKT)
CDR = (HCM, DGT)
MKT = (RGP, CQR)
MKK = (RMJ, DBK)
AAA = (TFQ, VSD)
QHL = (JBG, RNL)
LXJ = (HPJ, QXC)
QLT = (HQL, XDP)
FHN = (MMF, CKL)
SHF = (FHQ, FHG)
MTJ = (SJT, KFG)
DPJ = (SVF, HPV)
FPX = (BNR, RSG)
CVL = (XRS, HHM)
NSK = (DBN, SBK)
QNF = (PQK, RJC)
KKJ = (BVJ, NTK)
XNP = (KPD, QTF)
XTJ = (HNS, XBV)
KQT = (BHG, KTL)
BPL = (QRS, PJB)
JSX = (LCN, VQK)
MLC = (JLF, JLF)
QBX = (VVX, TNJ)
PGP = (CMN, SRR)
HRX = (HFM, JDB)
GGK = (SQS, LLM)
HVV = (VRG, JNJ)
RND = (HHM, XRS)
RPQ = (RMQ, JMH)
LRF = (QBX, CNL)
GJN = (TLF, LNB)
CTJ = (HPT, MBC)
HSR = (DKT, GFK)
KTL = (PMJ, CRN)
BDF = (LRF, PCQ)
PFS = (XBP, NQC)
VRQ = (KBL, VVD)
QRR = (VNR, LGB)
BNJ = (PVP, HPC)
PQC = (GPT, HFK)
CVX = (KKF, PFS)
BTQ = (BLC, VGC)
SDG = (LXT, QRR)
RJM = (FNN, QJJ)
JRF = (NSK, LFQ)
DFB = (DKJ, MGR)
XSC = (CFX, DHF)
JKH = (QRS, PJB)
NQQ = (NCR, CGS)
JHV = (CTJ, SLC)
LBB = (NVS, QJG)
SBK = (VDL, NPF)
HLD = (LXJ, XQB)
XCR = (DJM, PPS)
KPD = (GDF, KJL)
HKL = (VQK, LCN)
JKT = (BLH, CCN)
SRQ = (XFS, QHQ)
LXT = (VNR, LGB)
PJT = (TTC, QLT)
VNC = (BQR, HCT)
XNQ = (TDD, QFZ)
FDN = (MBR, DVG)
JBL = (MNK, VPC)
NRX = (PVH, PVH)
TTK = (VQQ, FSH)
XSJ = (BFQ, RDK)
VTP = (NXM, MRQ)
BCP = (MLC, PRC)
JQS = (LRF, PCQ)
QJC = (RJR, CNX)
KSP = (MHL, QJH)
KPN = (TKL, HKQ)
LCN = (KXQ, FBF)
GLG = (XVX, CQQ)
JTM = (FJH, DDP)
RQH = (PHJ, PKD)
TVV = (HJV, JGJ)
NKQ = (KKK, DSB)
GXS = (PVP, HPC)
SPS = (HVX, CFR)
GXD = (MFV, DMH)
QXJ = (HVT, MSF)
PTP = (MTT, VFN)
RMB = (DXR, RKJ)
NTN = (JMH, RMQ)
GJV = (JDH, NQQ)
JFV = (PKD, PHJ)
BMB = (MLB, QJL)
DFC = (GFH, SMN)
FFK = (XNT, JSQ)
PHJ = (VVL, RMS)
QRS = (RMC, VCJ)
PNX = (BXH, JLS)
GNB = (VHB, HFG)
QJN = (LXX, MRV)
MDM = (VCR, JHD)
BSR = (LXG, LST)
HXJ = (SRQ, HND)
HCT = (TTX, RNP)
HJV = (KCQ, QCB)
VXK = (FFK, JVQ)
GNP = (KQX, TSS)
RKD = (FCR, BXX)
FXP = (BXX, FCR)
FSH = (QSB, FPX)
QJJ = (DVB, GLR)
XPC = (KKJ, GNV)
HVQ = (NMR, DRN)
RSG = (DLB, QNM)
JSQ = (HMS, RJH)
FHS = (DHS, CDT)
JQB = (GQR, GVT)
QXC = (KTG, TTD)
JTL = (QLS, XGR)
VNR = (QFF, VNC)
PHG = (FTJ, TVR)
PMJ = (PJV, PJV)
SRJ = (LCV, JCL)
MDF = (XBB, QTH)
NDS = (SRF, RQP)
RHP = (XQQ, SHF)
NBL = (PXH, KGQ)
JGS = (HFC, FJJ)
QRD = (RGF, FMN)
NDX = (SHX, HBB)
LXG = (XMJ, RQS)
NGH = (RLX, CDR)
DJB = (FLC, RJS)
TTD = (MKK, BRS)
XVZ = (QSX, DJF)
BQV = (NQQ, JDH)
TTC = (HQL, XDP)
DHF = (BSJ, MCL)
VLV = (LSH, KTF)
QLS = (TCX, BXB)
QCT = (HFK, GPT)
XDP = (JQX, SVH)
LQL = (XCR, GCH)
MXH = (VHB, HFG)
HQB = (DMN, TNR)
CKL = (VVR, PSF)
KVG = (KMF, CRS)
MXT = (TLX, FQN)
VTV = (JHV, FDL)
TTT = (KPD, QTF)
PLN = (LMM, KCF)
VCJ = (QHM, PTR)
VCX = (XKM, RBG)
RDK = (VDG, RFD)
JCC = (QJL, MLB)
RFD = (GXS, BNJ)
RQP = (NQH, PKG)
GST = (PNB, DVN)
KFR = (JBP, GLM)
PPS = (XXC, FDG)
VJL = (PGH, PGH)
TVR = (LJN, QJV)
SLR = (BDH, QJC)
SMM = (KGR, TTK)
HGH = (QKX, KJP)
GDD = (PQS, XPC)
RPB = (TLD, LPB)
KKF = (XBP, NQC)
SCB = (MBR, MBR)
PPC = (LXT, QRR)
TFT = (XPK, SHP)
XQQ = (FHG, FHQ)
PVH = (TDD, TDD)
SJT = (QNP, VTP)
RRS = (JPC, JQB)
CMR = (JQM, GDS)
KHC = (BDF, JQS)
BLH = (KXD, SKF)
BFN = (LPB, TLD)
LSH = (NTN, RPQ)
NMN = (DRN, NMR)
QJL = (VHG, VVC)
XNJ = (SMN, GFH)
RGF = (HLD, RVL)
NGF = (PGQ, NLV)
RMX = (LXX, MRV)
QDK = (BCP, JNX)
LVL = (SPS, KQJ)
QFP = (SSV, QCP)
DNT = (CQX, XSJ)
MCL = (DCV, QBR)
HNJ = (PRQ, KKR)
QNM = (DBX, FNP)
XLQ = (TKL, HKQ)
JTD = (HXL, VTV)
KXD = (XSH, PNX)
THC = (JRF, FSV)
VCR = (FXP, RKD)
KKK = (DNG, DNM)
PKQ = (BLC, VGC)
PGQ = (DNT, PNK)
DMH = (NJG, RVM)
VVL = (JLT, GQH)
RFN = (QHL, CXS)
RKR = (PRQ, KKR)
XNT = (HMS, RJH)
JLL = (RTH, BNG)
HCM = (TDX, VGM)
NVH = (HNS, XBV)
HFM = (MST, PTP)
LJF = (XLR, KHC)
GSV = (JLG, QQC)
STF = (KGD, NSJ)
DHB = (FHL, GPM)
FJC = (QCL, HGP)
BPN = (MVD, LQF)
KSR = (JPC, JQB)
KGD = (VLH, VLH)
DPN = (TFT, LGT)
QHQ = (QPV, JLL)
FJH = (XFV, NMV)
QBP = (XXJ, KVG)
PNK = (CQX, XSJ)
TBN = (SPT, JCV)
DVG = (HRX, VGZ)
NXH = (JTD, SCS)
DDD = (BJC, JTM)
HFC = (SDG, PPC)
QKX = (XPM, QRP)
GFK = (GCL, HGH)
LMM = (MPD, BMK)
PKG = (HLC, XSD)
MLB = (VVC, VHG)
BMK = (JKH, BPL)
HBN = (XTT, LXV)
QMN = (JTR, PHF)
TLM = (DHS, CDT)
PPZ = (HCG, NGT)
VPK = (QJD, KCK)
BRS = (RMJ, DBK)
RQS = (XNP, TTT)
NNM = (RRR, NDS)
SRR = (HBN, QDP)
TTN = (XLQ, KPN)
LDL = (QXJ, FQP)
PQT = (PGH, PPZ)
BLM = (LGT, TFT)
FMQ = (BTQ, PKQ)
KQD = (TSS, KQX)
XLR = (BDF, JQS)
PKL = (LMK, FJS)
KCF = (MPD, BMK)
MHD = (KBG, SKT)
RMQ = (THC, LFF)
QPM = (JGF, TDH)
QCP = (VPB, PKL)
LCV = (LCJ, XQM)
BCD = (PKQ, BTQ)
RBG = (MLQ, STS)
LVT = (DKT, GFK)
DKJ = (LKN, CBS)
DKB = (HDL, QFP)
HPV = (RMX, QJN)
MDT = (RCH, CMR)
CFQ = (LXG, LST)
XDV = (FQN, TLX)
KXQ = (VRH, VRQ)
CVB = (DSB, KKK)
DBX = (DHB, QHF)
DSB = (DNM, DNG)
CFX = (BSJ, MCL)
KJL = (QDK, VKJ)
RTP = (KGQ, PXH)
FJS = (QTR, FJC)
HGJ = (VKQ, DCR)
NXM = (LNC, HXR)
CQX = (BFQ, RDK)
BVV = (NQB, GMG)
RRL = (DHF, CFX)
JMH = (THC, LFF)
DVH = (CCV, DLR)
JFX = (RTB, XBH)
GLR = (NXH, BLP)
RMC = (QHM, PTR)
NQB = (SXX, FSL)
CTB = (CKS, XQD)
VSJ = (QCT, PQC)
NCR = (CNR, VQX)
JTR = (MDT, LFL)
DGT = (TDX, VGM)
KFM = (CTB, LFG)
SQG = (FPN, NHP)
RTB = (XVQ, MDF)
DSC = (GXD, PLD)
PRB = (XDQ, XLJ)
LSV = (LCP, VHS)
CCQ = (MLM, TKV)
DRS = (DPN, BLM)
RVL = (LXJ, XQB)
DKT = (HGH, GCL)
GCP = (LCP, VHS)
MPD = (JKH, BPL)
BVJ = (PCK, HMC)
LST = (XMJ, RQS)
LRB = (NGH, BVD)
MNM = (JGJ, HJV)
QSB = (BNR, RSG)
BVB = (BGT, CLM)
NMV = (JCC, BMB)
HVX = (GDD, BJS)
FBL = (BJC, JTM)
SFR = (RGF, FMN)
KCK = (JRD, NFC)
FTJ = (LJN, QJV)
VSD = (SBB, GST)
MSF = (RCJ, NDX)
CNL = (VVX, TNJ)
CLH = (JBM, JSB)
JRQ = (TGH, LFN)
HCV = (QPM, FXJ)
NMR = (RRL, XSC)
BGT = (RJM, NMQ)
NQC = (KND, CRG)
BXV = (KSP, MRN)
SCS = (HXL, VTV)
JCL = (LCJ, XQM)
JTQ = (NLV, PGQ)
TSS = (LVT, HSR)
CDT = (KNG, VLV)
QBR = (QMN, XXH)
QJD = (NFC, JRD)
JDH = (CGS, NCR)
SLC = (MBC, HPT)
BLC = (BSR, CFQ)
JJN = (BCD, FMQ)
DXR = (SCB, SCB)
CRG = (JKT, VKS)
DNG = (KXS, FQX)
GCL = (QKX, KJP)
SPT = (KSK, KSK)
PQK = (XTJ, NVH)
NPF = (PRB, KLG)
RCJ = (HBB, SHX)
GCH = (PPS, DJM)
QRA = (DJF, QSX)
DJF = (MDM, LKV)
RRR = (SRF, RQP)
TLD = (RVV, HGJ)
LNC = (XGJ, VSJ)
VVD = (TTN, PRD)
TNJ = (XLF, CLG)
JNJ = (LVK, LBB)
GXH = (NRX, TNT)
DFR = (PGP, TCC)
RCH = (JQM, GDS)
FMN = (HLD, RVL)
TFQ = (GST, SBB)
NSQ = (FQP, QXJ)
LFF = (JRF, FSV)
HVT = (RCJ, NDX)
SHR = (KFM, TVX)
SQS = (VKX, VTT)
PHF = (LFL, MDT)
PJV = (SPT, SPT)
LFG = (CKS, XQD)
HND = (QHQ, XFS)
FFD = (BGT, CLM)
KMF = (LQL, FHT)
XVQ = (XBB, QTH)
JCV = (KSK, ZZZ)
JVQ = (JSQ, XNT)
DHS = (KNG, VLV)
SKF = (PNX, XSH)
BJC = (DDP, FJH)
GGD = (RQH, JFV)
JBP = (BVB, FFD)
JRJ = (NBL, RTP)
HMK = (LJF, QQZ)
JBG = (GGK, NHR)
XQB = (QXC, HPJ)
NTK = (PCK, HMC)
RTH = (BQV, GJV)
LFQ = (DBN, SBK)
KKR = (RJB, PKR)
DRK = (DRS, NVJ)
SHP = (SMM, SSN)
KCQ = (LSV, GCP)
KTG = (BRS, MKK)
GJT = (DPJ, KTV)
JPN = (DMN, TNR)
HNS = (GRL, TKH)
KBG = (GGD, BXR)
RVM = (QNF, MQF)
KQA = (XLR, KHC)
LNB = (MXT, XDV)
KGR = (VQQ, FSH)
BXR = (RQH, JFV)
MNK = (XNJ, DFC)
CQR = (SFR, QRD)
MTT = (SRB, JBL)
CQQ = (QKG, GSV)
MTH = (BHG, KTL)
MBC = (FXS, CCQ)
CLM = (RJM, NMQ)
VGZ = (JDB, HFM)
PRC = (JLF, NKV)
LPB = (HGJ, RVV)
QPV = (BNG, RTH)
ZZZ = (VSD, TFQ)
VKS = (BLH, CCN)
FDG = (MTL, RMB)
FQN = (NRN, LRB)
MGR = (LKN, CBS)
GMG = (SXX, FSL)
BVG = (BVV, CSL)
VQX = (JXS, JRJ)
DFA = (HFM, JDB)
RMS = (GQH, JLT)
XFV = (BMB, JCC)
BLP = (SCS, JTD)
HST = (DQH, GVC)
JLF = (VJL, VJL)
BQR = (TTX, RNP)
FDL = (SLC, CTJ)
HSQ = (SHR, VFR)
FPN = (DJB, MBX)
DMN = (GJT, HRJ)
MFV = (NJG, RVM)
VPB = (FJS, LMK)
KSK = (TFQ, VSD)
VRH = (VVD, KBL)
VTT = (QBP, HVJ)
PXH = (RND, CVL)
JLT = (PKV, GLG)
DVB = (BLP, NXH)
QTH = (FHN, JFF)
SSV = (VPB, PKL)
PTR = (KQD, GNP)
XRS = (JPN, HQB)
NLV = (PNK, DNT)
KQX = (HSR, LVT)
VPC = (XNJ, DFC)
PJB = (VCJ, RMC)
FHQ = (BHV, SBM)
NRN = (BVD, NGH)
TLF = (XDV, MXT)
MMF = (PSF, VVR)
QCB = (GCP, LSV)
RGP = (SFR, QRD)
DBA = (NGT, HCG)
JNV = (VRG, JNJ)
JLG = (GJN, LKQ)
DDP = (NMV, XFV)
BHV = (CFJ, GRP)
HXL = (JHV, FDL)
JDB = (PTP, MST)
HFK = (DTK, PLF)
KQJ = (HVX, CFR)
MHL = (PMR, DKB)
GVL = (RBG, XKM)
TNR = (HRJ, GJT)
FQP = (MSF, HVT)
FQX = (KJT, SRJ)
FJJ = (SDG, PPC)
LCJ = (XQS, STF)
SVF = (RMX, QJN)
CXS = (JBG, RNL)
NJP = (FPN, NHP)
TNH = (KSP, MRN)
CSL = (NQB, GMG)
NFC = (FBL, DDD)
QFZ = (DSC, DKD)
XQS = (KGD, KGD)
BJS = (XPC, PQS)
JGJ = (KCQ, QCB)
XSD = (TLM, FHS)
QHM = (KQD, GNP)
XBP = (KND, CRG)
KFG = (QNP, VTP)
JRL = (FXJ, QPM)
XSH = (BXH, JLS)
BDH = (CNX, RJR)
GNV = (BVJ, NTK)
SBM = (GRP, CFJ)
BXH = (JGS, XRM)
GQH = (GLG, PKV)
FNP = (DHB, QHF)
TDH = (HGM, MTJ)
VKX = (HVJ, QBP)
PLF = (BJH, DFB)
FSL = (HCV, JRL)
VQK = (KXQ, FBF)
BJJ = (HND, SRQ)
SRT = (DJF, QSX)
LGT = (XPK, SHP)
CFR = (BJS, GDD)
LFL = (CMR, RCH)
SHX = (HXJ, BJJ)
VHG = (VFJ, RHP)
BXX = (BXV, TNH)
JBM = (HNL, JFX)
CKS = (GBT, CVX)
HVJ = (KVG, XXJ)
CMN = (QDP, HBN)
HRJ = (DPJ, KTV)
FSV = (NSK, LFQ)
FBF = (VRH, VRQ)
GRP = (PMT, DFF)
VFN = (SRB, JBL)
HMC = (JRQ, SNK)
TCC = (SRR, CMN)
XBH = (XVQ, MDF)
TVX = (CTB, LFG)
DQH = (MDC, HSQ)
DVN = (PHG, DSS)
QCV = (DQH, GVC)
NVJ = (BLM, DPN)
KXS = (SRJ, KJT)
QRP = (CMS, JTL)
MBR = (HRX, HRX)
SRF = (PKG, NQH)
VHB = (NJP, SQG)
PRQ = (PKR, RJB)
BNG = (BQV, GJV)
TTX = (NNM, LFH)
XDQ = (CVB, NKQ)
MRV = (JSX, HKL)
QQZ = (KHC, XLR)
XVX = (GSV, QKG)
RJC = (NVH, XTJ)
GLK = (PGP, TCC)
XBV = (GRL, TKH)
STS = (CJK, VXK)
XTT = (RKR, HNJ)
SBB = (DVN, PNB)
JRD = (DDD, FBL)
GRL = (MNM, TVV)
SMR = (CSL, BVV)
DCV = (XXH, QMN)
GFH = (JTQ, NGF)
HXR = (VSJ, XGJ)
CCN = (KXD, SKF)
SXX = (HCV, JRL)
RJH = (KMT, VPK)
RJS = (VCX, GVL)
TLX = (LRB, NRN)
SSN = (KGR, TTK)
XLF = (QCV, HST)
SVH = (SLR, JBK)
RJB = (STR, BJQ)
RMJ = (RFN, DXF)
LGC = (FMQ, BCD)
VRG = (LBB, LVK)
HGM = (SJT, KFG)
JFF = (CKL, MMF)
QQC = (GJN, LKQ)
PLD = (MFV, DMH)
DSS = (FTJ, TVR)
NKV = (VJL, PQT)
CBS = (BRN, GXH)
JBK = (BDH, QJC)
HKQ = (RRS, KSR)
GQR = (DVH, TRF)
BJH = (DKJ, MGR)
TKL = (KSR, RRS)
BFQ = (RFD, VDG)
LCP = (LVL, XDL)
FCR = (BXV, TNH)
HFG = (NJP, SQG)
GPM = (PLN, SSL)
NHR = (SQS, LLM)
CLG = (QCV, HST)
PKV = (CQQ, XVX)
JLS = (XRM, JGS)
PRD = (XLQ, KPN)
HLC = (TLM, FHS)
QDP = (XTT, LXV)
DXF = (CXS, QHL)
KTF = (NTN, RPQ)
QJG = (JJN, LGC)
QCL = (MXH, GNB)
PCK = (JRQ, SNK)
HDL = (SSV, QCP)
FNN = (DVB, GLR)
HGP = (GNB, MXH)
VGC = (BSR, CFQ)
XRM = (HFC, FJJ)
DLB = (DBX, FNP)
LFH = (NDS, RRR)
RNL = (GGK, NHR)
DBK = (DXF, RFN)
HPT = (CCQ, FXS)
LXV = (RKR, HNJ)
HQL = (JQX, SVH)
TRF = (DLR, CCV)
QJV = (HVQ, NMN)
VFR = (TVX, KFM)
HNL = (XBH, RTB)
LJN = (HVQ, NMN)
GVT = (DVH, TRF)
FHT = (XCR, GCH)
HCG = (PJD, CLH)
QKG = (QQC, JLG)
MRQ = (LNC, HXR)
FLC = (GVL, VCX)
PNB = (PHG, DSS)
VVR = (NSQ, LDL)
QJH = (DKB, PMR)
VFJ = (XQQ, SHF)
SSL = (LMM, KCF)
MST = (MTT, VFN)
PVP = (SND, MKT)
HBB = (BJJ, HXJ)
PMR = (HDL, QFP)
KTV = (HPV, SVF)
XXJ = (CRS, KMF)
CGS = (CNR, VQX)
SNK = (TGH, LFN)
GPT = (DTK, PLF)
RVV = (VKQ, DCR)
KBL = (TTN, PRD)
JSB = (HNL, JFX)
JPC = (GQR, GVT)
NGT = (CLH, PJD)
LQF = (SRT, XVZ)
LMK = (QTR, FJC)
QTR = (QCL, HGP)
XBB = (FHN, JFF)
NJG = (MQF, QNF)
XLJ = (CVB, NKQ)
LGB = (QFF, VNC)
NQK = (JBP, GLM)
LVK = (NVS, QJG)
HJA = (DKD, DSC)
XXH = (PHF, JTR)
BHG = (PMJ, CRN)
KJT = (LCV, JCL)
MQF = (RJC, PQK)
KLG = (XDQ, XLJ)
XXC = (MTL, RMB)
PGH = (NGT, HCG)
XPK = (SSN, SMM)
BRN = (NRX, NRX)
MRN = (QJH, MHL)
QSX = (MDM, LKV)
SRB = (VPC, MNK)
VGM = (MQR, BPN)
JNX = (MLC, PRC)
VQQ = (QSB, FPX)
DRN = (XSC, RRL)
LKQ = (LNB, TLF)
LKV = (VCR, JHD)
MTL = (DXR, RKJ)
PKD = (RMS, VVL)
DNM = (KXS, FQX)
FXS = (MLM, TKV)
CFJ = (PMT, DFF)
XFS = (JLL, QPV)
JXS = (RTP, NBL)
TCX = (KQT, MTH)
TGH = (DFR, GLK)
JHD = (RKD, FXP)
MDC = (VFR, SHR)
XGR = (BXB, TCX)
RJR = (CTN, DRK)
LKN = (BRN, GXH)
DFF = (KVT, MHD)
GDF = (VKJ, QDK)
PSF = (LDL, NSQ)
HPC = (SND, MKT)
CRN = (PJV, TBN)
KMT = (KCK, QJD)
QTF = (KJL, GDF)
XQM = (XQS, STF)
TKH = (MNM, TVV)
KGQ = (CVL, RND)
GBT = (PFS, KKF)
DTK = (BJH, DFB)
TDD = (DKD, DSC)
BXB = (MTH, KQT)
DCR = (HVV, JNV)
VLH = (LJF, LJF)
GLM = (BVB, FFD)
CCV = (BFN, RPB)
MBX = (RJS, FLC)
CMS = (QLS, XGR)
XMJ = (TTT, XNP)
JGF = (MTJ, HGM)
VDG = (GXS, BNJ)
LFN = (DFR, GLK)
TDX = (MQR, MQR)
PMT = (KVT, MHD)
PKR = (BJQ, STR)
XDL = (KQJ, SPS)
XKM = (MLQ, STS)
XGJ = (PQC, QCT)
MLM = (BVG, SMR)
PJD = (JBM, JSB)
GVC = (MDC, HSQ)
NVS = (JJN, LGC)
RLX = (HCM, DGT)
BVD = (CDR, RLX)
RNP = (NNM, LFH)
TNT = (PVH, XNQ)
KJP = (XPM, QRP)
NMQ = (QJJ, FNN)
XQD = (CVX, GBT)
SMN = (NGF, JTQ)
DLR = (BFN, RPB)
MQR = (MVD, MVD)
NHP = (DJB, MBX)
DBD = (QLT, TTC)
DJM = (FDG, XXC)
CNR = (JXS, JRJ)
VVC = (RHP, VFJ)
BJQ = (NQK, KFR)
HHM = (JPN, HQB)
NQH = (HLC, XSD)
QHF = (GPM, FHL)
BSJ = (QBR, DCV)
JQX = (SLR, JBK)
XPM = (JTL, CMS)
CJK = (FFK, JVQ)
VVX = (CLG, XLF)
HMS = (VPK, KMT)
VKJ = (BCP, JNX)
LXX = (JSX, HKL)
CRS = (FHT, LQL)
FXJ = (TDH, JGF)
FHG = (SBM, BHV)
HPJ = (KTG, TTD)
VKQ = (JNV, HVV)
KNG = (LSH, KTF)
SND = (RGP, CQR)
RKJ = (SCB, FDN)
DKD = (GXD, PLD)";