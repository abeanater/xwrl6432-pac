#[doc = "Register `LNK` reader"]
pub type R = crate::R<LnkSpec>;
#[doc = "Register `LNK` writer"]
pub type W = crate::W<LnkSpec>;
#[doc = "Field `LINK` reader - 15:0\\]
Link Address: The CC provides a mechanism to reload the current PaRAM Entry upon its natural termination (i.e. after count fields are decremented to '0') with a new PaRAM Entry. This is called 'linking'. The 16-bit parameter LINK specifies the byte address offset in the PaRAM from which the CC loads/reloads the next PaRAM entry in the link. The CC should disregard the value in the upper 2 bits of the LINK field as well as the lower 5-bits of the LINK field. The upper two bits are ignored such that the user can program either the 'literal' byte address of the LINK parameter or the 'PaRAM base-relative' address of the link field. Therefore if the user uses the literal address with a range from 0x4000 to 0x7FFF it will be treated as a PaRAM-base-relative value of 0x0000 to 0x3FFF. The lower-5 bits are ignored and treated as 'b00000 thereby guaranteeing that all Link pointers point to a 32-byte aligned PaRAM entry. In the latter case (5-lsbs) behavior is undefined for the user (i.e. don't have to test it). In the former case (2 msbs) user should be able to take advantage of this feature (i.e. do have to test it). If a Link Update is requested to a PaRAM address that is beyond the actual range of implemented PaRAM then the Link will be treated as a Null Linkand all 0s plus 0xFFFF will be written to the current entry location. A LINK value of 0xFFFF is referred to as a NULL link which should cause the CC to write 0x0 to all entries of the current PaRAM Entry except for the LINK field which is set to 0xFFFF. The Priv/Privid/Secure state is overwritten to 0x0 when linking. MSBs and LSBS should not be masked when comparing against the 0xFFFF value. I.e. a value of 0x3FFE is a non-NULL PaRAM link field."]
pub type LinkR = crate::FieldReader<u16>;
#[doc = "Field `LINK` writer - 15:0\\]
Link Address: The CC provides a mechanism to reload the current PaRAM Entry upon its natural termination (i.e. after count fields are decremented to '0') with a new PaRAM Entry. This is called 'linking'. The 16-bit parameter LINK specifies the byte address offset in the PaRAM from which the CC loads/reloads the next PaRAM entry in the link. The CC should disregard the value in the upper 2 bits of the LINK field as well as the lower 5-bits of the LINK field. The upper two bits are ignored such that the user can program either the 'literal' byte address of the LINK parameter or the 'PaRAM base-relative' address of the link field. Therefore if the user uses the literal address with a range from 0x4000 to 0x7FFF it will be treated as a PaRAM-base-relative value of 0x0000 to 0x3FFF. The lower-5 bits are ignored and treated as 'b00000 thereby guaranteeing that all Link pointers point to a 32-byte aligned PaRAM entry. In the latter case (5-lsbs) behavior is undefined for the user (i.e. don't have to test it). In the former case (2 msbs) user should be able to take advantage of this feature (i.e. do have to test it). If a Link Update is requested to a PaRAM address that is beyond the actual range of implemented PaRAM then the Link will be treated as a Null Linkand all 0s plus 0xFFFF will be written to the current entry location. A LINK value of 0xFFFF is referred to as a NULL link which should cause the CC to write 0x0 to all entries of the current PaRAM Entry except for the LINK field which is set to 0xFFFF. The Priv/Privid/Secure state is overwritten to 0x0 when linking. MSBs and LSBS should not be masked when comparing against the 0xFFFF value. I.e. a value of 0x3FFE is a non-NULL PaRAM link field."]
pub type LinkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BCNTRLD` reader - 31:16\\]
BCNT Reload: BCNTRLD is a 16-bit unsigned value used to reload the BCNT field once the last array in the 2nd dimension is transferred. This field is only used for A-Sync'ed transfers. In this case the CC decrements the BCNT value by one on each TR submission. When BCNT (conceptually) reaches zero then the CC decrements CCNT and uses the BCNTRLD value to reinitialize the BCNT value. For AB-synchronized transfers the CC submits the BCNT in the TR and therefore the TC is responsible to keep track of BCNT not thus BCNTRLD is a don't care field."]
pub type BcntrldR = crate::FieldReader<u16>;
#[doc = "Field `BCNTRLD` writer - 31:16\\]
BCNT Reload: BCNTRLD is a 16-bit unsigned value used to reload the BCNT field once the last array in the 2nd dimension is transferred. This field is only used for A-Sync'ed transfers. In this case the CC decrements the BCNT value by one on each TR submission. When BCNT (conceptually) reaches zero then the CC decrements CCNT and uses the BCNTRLD value to reinitialize the BCNT value. For AB-synchronized transfers the CC submits the BCNT in the TR and therefore the TC is responsible to keep track of BCNT not thus BCNTRLD is a don't care field."]
pub type BcntrldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Link Address: The CC provides a mechanism to reload the current PaRAM Entry upon its natural termination (i.e. after count fields are decremented to '0') with a new PaRAM Entry. This is called 'linking'. The 16-bit parameter LINK specifies the byte address offset in the PaRAM from which the CC loads/reloads the next PaRAM entry in the link. The CC should disregard the value in the upper 2 bits of the LINK field as well as the lower 5-bits of the LINK field. The upper two bits are ignored such that the user can program either the 'literal' byte address of the LINK parameter or the 'PaRAM base-relative' address of the link field. Therefore if the user uses the literal address with a range from 0x4000 to 0x7FFF it will be treated as a PaRAM-base-relative value of 0x0000 to 0x3FFF. The lower-5 bits are ignored and treated as 'b00000 thereby guaranteeing that all Link pointers point to a 32-byte aligned PaRAM entry. In the latter case (5-lsbs) behavior is undefined for the user (i.e. don't have to test it). In the former case (2 msbs) user should be able to take advantage of this feature (i.e. do have to test it). If a Link Update is requested to a PaRAM address that is beyond the actual range of implemented PaRAM then the Link will be treated as a Null Linkand all 0s plus 0xFFFF will be written to the current entry location. A LINK value of 0xFFFF is referred to as a NULL link which should cause the CC to write 0x0 to all entries of the current PaRAM Entry except for the LINK field which is set to 0xFFFF. The Priv/Privid/Secure state is overwritten to 0x0 when linking. MSBs and LSBS should not be masked when comparing against the 0xFFFF value. I.e. a value of 0x3FFE is a non-NULL PaRAM link field."]
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
BCNT Reload: BCNTRLD is a 16-bit unsigned value used to reload the BCNT field once the last array in the 2nd dimension is transferred. This field is only used for A-Sync'ed transfers. In this case the CC decrements the BCNT value by one on each TR submission. When BCNT (conceptually) reaches zero then the CC decrements CCNT and uses the BCNTRLD value to reinitialize the BCNT value. For AB-synchronized transfers the CC submits the BCNT in the TR and therefore the TC is responsible to keep track of BCNT not thus BCNTRLD is a don't care field."]
    #[inline(always)]
    pub fn bcntrld(&self) -> BcntrldR {
        BcntrldR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Link Address: The CC provides a mechanism to reload the current PaRAM Entry upon its natural termination (i.e. after count fields are decremented to '0') with a new PaRAM Entry. This is called 'linking'. The 16-bit parameter LINK specifies the byte address offset in the PaRAM from which the CC loads/reloads the next PaRAM entry in the link. The CC should disregard the value in the upper 2 bits of the LINK field as well as the lower 5-bits of the LINK field. The upper two bits are ignored such that the user can program either the 'literal' byte address of the LINK parameter or the 'PaRAM base-relative' address of the link field. Therefore if the user uses the literal address with a range from 0x4000 to 0x7FFF it will be treated as a PaRAM-base-relative value of 0x0000 to 0x3FFF. The lower-5 bits are ignored and treated as 'b00000 thereby guaranteeing that all Link pointers point to a 32-byte aligned PaRAM entry. In the latter case (5-lsbs) behavior is undefined for the user (i.e. don't have to test it). In the former case (2 msbs) user should be able to take advantage of this feature (i.e. do have to test it). If a Link Update is requested to a PaRAM address that is beyond the actual range of implemented PaRAM then the Link will be treated as a Null Linkand all 0s plus 0xFFFF will be written to the current entry location. A LINK value of 0xFFFF is referred to as a NULL link which should cause the CC to write 0x0 to all entries of the current PaRAM Entry except for the LINK field which is set to 0xFFFF. The Priv/Privid/Secure state is overwritten to 0x0 when linking. MSBs and LSBS should not be masked when comparing against the 0xFFFF value. I.e. a value of 0x3FFE is a non-NULL PaRAM link field."]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LinkW<LnkSpec> {
        LinkW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
BCNT Reload: BCNTRLD is a 16-bit unsigned value used to reload the BCNT field once the last array in the 2nd dimension is transferred. This field is only used for A-Sync'ed transfers. In this case the CC decrements the BCNT value by one on each TR submission. When BCNT (conceptually) reaches zero then the CC decrements CCNT and uses the BCNTRLD value to reinitialize the BCNT value. For AB-synchronized transfers the CC submits the BCNT in the TR and therefore the TC is responsible to keep track of BCNT not thus BCNTRLD is a don't care field."]
    #[inline(always)]
    #[must_use]
    pub fn bcntrld(&mut self) -> BcntrldW<LnkSpec> {
        BcntrldW::new(self, 16)
    }
}
#[doc = "Link and Reload parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`lnk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lnk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LnkSpec;
impl crate::RegisterSpec for LnkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lnk::R`](R) reader structure"]
impl crate::Readable for LnkSpec {}
#[doc = "`write(|w| ..)` method takes [`lnk::W`](W) writer structure"]
impl crate::Writable for LnkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LNK to value 0"]
impl crate::Resettable for LnkSpec {
    const RESET_VALUE: u32 = 0;
}
