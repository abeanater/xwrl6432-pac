#[doc = "Register `APPSS_TPTC_XID_REORDER_CFG` reader"]
pub type R = crate::R<AppssTptcXidReorderCfgSpec>;
#[doc = "Register `APPSS_TPTC_XID_REORDER_CFG` writer"]
pub type W = crate::W<AppssTptcXidReorderCfgSpec>;
#[doc = "Field `tptc_a0_disable` reader - 0:0\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_A0"]
pub type TptcA0DisableR = crate::BitReader;
#[doc = "Field `tptc_a0_disable` writer - 0:0\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_A0"]
pub type TptcA0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a1_disable` reader - 8:8\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_A1"]
pub type TptcA1DisableR = crate::BitReader;
#[doc = "Field `tptc_a1_disable` writer - 8:8\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_A1"]
pub type TptcA1DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_disable` reader - 16:16\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_B0"]
pub type TptcB0DisableR = crate::BitReader;
#[doc = "Field `tptc_b0_disable` writer - 16:16\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_B0"]
pub type TptcB0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1_disable` reader - 24:24\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_B1"]
pub type TptcB1DisableR = crate::BitReader;
#[doc = "Field `tptc_b1_disable` writer - 24:24\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_B1"]
pub type TptcB1DisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_A0"]
    #[inline(always)]
    pub fn tptc_a0_disable(&self) -> TptcA0DisableR {
        TptcA0DisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_A1"]
    #[inline(always)]
    pub fn tptc_a1_disable(&self) -> TptcA1DisableR {
        TptcA1DisableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_B0"]
    #[inline(always)]
    pub fn tptc_b0_disable(&self) -> TptcB0DisableR {
        TptcB0DisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_B1"]
    #[inline(always)]
    pub fn tptc_b1_disable(&self) -> TptcB1DisableR {
        TptcB1DisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_A0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_disable(&mut self) -> TptcA0DisableW<AppssTptcXidReorderCfgSpec> {
        TptcA0DisableW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_A1"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_disable(&mut self) -> TptcA1DisableW<AppssTptcXidReorderCfgSpec> {
        TptcA1DisableW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_B0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_disable(&mut self) -> TptcB0DisableW<AppssTptcXidReorderCfgSpec> {
        TptcB0DisableW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for TPTC_B1"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_disable(&mut self) -> TptcB1DisableW<AppssTptcXidReorderCfgSpec> {
        TptcB1DisableW::new(self, 24)
    }
}
#[doc = "APPSS_TPTC_XID_REORDER_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_tptc_xid_reorder_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_tptc_xid_reorder_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssTptcXidReorderCfgSpec;
impl crate::RegisterSpec for AppssTptcXidReorderCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_tptc_xid_reorder_cfg::R`](R) reader structure"]
impl crate::Readable for AppssTptcXidReorderCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_tptc_xid_reorder_cfg::W`](W) writer structure"]
impl crate::Writable for AppssTptcXidReorderCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_TPTC_XID_REORDER_CFG to value 0"]
impl crate::Resettable for AppssTptcXidReorderCfgSpec {
    const RESET_VALUE: u32 = 0;
}
