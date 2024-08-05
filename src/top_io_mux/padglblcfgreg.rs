#[doc = "Register `PADGLBLCFGREG` reader"]
pub type R = crate::R<PadglblcfgregSpec>;
#[doc = "Register `PADGLBLCFGREG` writer"]
pub type W = crate::W<PadglblcfgregSpec>;
#[doc = "Field `PADGLBLCFGREG` reader - 31:0\\]
2:0 : global_ie_n_ctl - Write 3'b111 to pass global_ie_n_val to IE_N/RXACTIVE_N pin of all the IOs. 3 : global_ie_n_val - Active low 10:8 : global_oe_n_ctl - Write 3'b111 to pass global_oe_n_val to OE_N/GZ pin of all the IOs. 11 : global_oe_n_val - Active low 18:16 : global_pi_ctl - Write 3'b111 to pass global_pi_val and global_pu_val to all the IOs 19 : global_pi_val 20 : global_pu_val"]
pub type PadglblcfgregR = crate::FieldReader<u32>;
#[doc = "Field `PADGLBLCFGREG` writer - 31:0\\]
2:0 : global_ie_n_ctl - Write 3'b111 to pass global_ie_n_val to IE_N/RXACTIVE_N pin of all the IOs. 3 : global_ie_n_val - Active low 10:8 : global_oe_n_ctl - Write 3'b111 to pass global_oe_n_val to OE_N/GZ pin of all the IOs. 11 : global_oe_n_val - Active low 18:16 : global_pi_ctl - Write 3'b111 to pass global_pi_val and global_pu_val to all the IOs 19 : global_pi_val 20 : global_pu_val"]
pub type PadglblcfgregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
2:0 : global_ie_n_ctl - Write 3'b111 to pass global_ie_n_val to IE_N/RXACTIVE_N pin of all the IOs. 3 : global_ie_n_val - Active low 10:8 : global_oe_n_ctl - Write 3'b111 to pass global_oe_n_val to OE_N/GZ pin of all the IOs. 11 : global_oe_n_val - Active low 18:16 : global_pi_ctl - Write 3'b111 to pass global_pi_val and global_pu_val to all the IOs 19 : global_pi_val 20 : global_pu_val"]
    #[inline(always)]
    pub fn padglblcfgreg(&self) -> PadglblcfgregR {
        PadglblcfgregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
2:0 : global_ie_n_ctl - Write 3'b111 to pass global_ie_n_val to IE_N/RXACTIVE_N pin of all the IOs. 3 : global_ie_n_val - Active low 10:8 : global_oe_n_ctl - Write 3'b111 to pass global_oe_n_val to OE_N/GZ pin of all the IOs. 11 : global_oe_n_val - Active low 18:16 : global_pi_ctl - Write 3'b111 to pass global_pi_val and global_pu_val to all the IOs 19 : global_pi_val 20 : global_pu_val"]
    #[inline(always)]
    #[must_use]
    pub fn padglblcfgreg(&mut self) -> PadglblcfgregW<PadglblcfgregSpec> {
        PadglblcfgregW::new(self, 0)
    }
}
#[doc = "PADGLBLCFGREG\n\nYou can [`read`](crate::Reg::read) this register and get [`padglblcfgreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padglblcfgreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadglblcfgregSpec;
impl crate::RegisterSpec for PadglblcfgregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padglblcfgreg::R`](R) reader structure"]
impl crate::Readable for PadglblcfgregSpec {}
#[doc = "`write(|w| ..)` method takes [`padglblcfgreg::W`](W) writer structure"]
impl crate::Writable for PadglblcfgregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADGLBLCFGREG to value 0"]
impl crate::Resettable for PadglblcfgregSpec {
    const RESET_VALUE: u32 = 0;
}
